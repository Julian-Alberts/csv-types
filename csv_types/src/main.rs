use std::{io, io::prelude::*};
use csv_types_sys as csvtypes;
use csvtypes::types;
use argparse::{ArgumentParser, StoreTrue, StoreOption, Store};
use std::fs;
use std::process;
mod print_result;

fn main() {
    let (config_file, asserted_types, options, machine_readable) = setup_args();

    let type_list = get_config(config_file);

    let input = read_input_from_stdin();
    
    if let Some(asserted_types) = asserted_types {
        assert_types(&input, type_list, options, asserted_types, machine_readable);
    } else {
        matching_types(&input, type_list, options, machine_readable);
    }

}

fn assert_types(csv: &str, type_list: types::TypeList, options: csvtypes::Options, asserted_types: String, machine_readable: bool) {
    let types_map = type_list.get_types_map();
    let mut expected_types = Vec::new();
    for type_name in asserted_types.split(',') {
       
        let type_name = type_name.trim();
        let expected = match types_map.get(type_name) {
            Some(t) => t.clone(),
            None => {
                eprintln!("The type {} is not defined", type_name);
                process::exit(1);
            }
        };
        expected_types.push(expected);
    }
    
    let rows = match csvtypes::assert_columns_match(csv, expected_types, options) {
        Ok(rows) => rows,
        Err(err) => {
            match err {
                csvtypes::Err::Join => eprintln!("Could not join threads."),
                csvtypes::Err::ThreadCount => eprintln!("The thread count needs to be bigger than 0"),
                csvtypes::Err::ColumnCountNotMatching => eprintln!("The given number of types does not match the number of columns"),
            }
            process::exit(1);
        }
    };

    print_result::assert_types(&rows[..], machine_readable);

}

fn matching_types(input: &str, type_list: types::TypeList, options: csvtypes::Options, machine_readable: bool) {
    let (headers, types) = match csvtypes::get_types(input, type_list, options) {
        Ok(r) => r,
        Err(err) => {
            match err {
                csvtypes::Err::Join => eprintln!("Could not join threads."),
                csvtypes::Err::ThreadCount => eprintln!("The thread count needs to be bigger than 0"),
                _ => eprintln!("An unknown Error accoured")
            }
            process::exit(1);
        }
    };

    print_result::matching_types(&types, &headers, machine_readable);
}

fn setup_args() -> (ConfigFileType, Option<String>, csvtypes::Options, bool) {
    let mut config_file_replace_default = String::new();
    let mut config_file = String::new();
    let mut options =  csvtypes::Options {
        has_headers: false,
        max_threads: None
    };
    let mut assert = None;
    let mut machine_readable = false;

    let mut ap = ArgumentParser::new();
    ap.refer(&mut options.has_headers)
    .add_option(&["--header"], StoreTrue, "File has header");
    ap.refer(&mut config_file)
    .add_option(&["-c", "--config-file"], Store, "Add custom types from file");
    ap.refer(&mut config_file_replace_default)
    .add_option(&["-C", "--config-file-replace-default"], Store, "Same as --config-file but replaces default config");
    ap.refer(&mut options.max_threads)
    .add_option(&["--max-threads"], StoreOption, "Maximal thread count");
    ap.refer(&mut assert)
    .add_option(&["--assert"], StoreOption, "Returns not matching rows and columns in pattern [row]:[column]:[column]...");
    ap.refer(&mut machine_readable)
    .add_option(&["-m"], StoreTrue, "Machine readable format");
    ap.parse_args_or_exit();
    
    drop(ap);

    if !config_file.is_empty() && !config_file_replace_default.is_empty() {
        eprintln!("You can only use on of --config-file --config-file-replace-default at a time");
        process::exit(1);
    }

    let config_file = if !config_file.is_empty() {
        ConfigFileType::Append(config_file)
    } else if !config_file_replace_default.is_empty() {
        ConfigFileType::ReplaceDefault(config_file_replace_default)
    } else {
        ConfigFileType::None
    };

    (config_file, assert, options, machine_readable)
}

fn get_config(config_file: ConfigFileType) -> types::TypeList {
    match config_file {
        ConfigFileType::None => types::TypeList::from(default_config()),
        ConfigFileType::ReplaceDefault(file) => types::TypeList::from(get_config_from_file(&file[..])),
        ConfigFileType::Append(file) => {
            let mut default = default_config();
            let mut file =  get_config_from_file(&file[..]);
            default.append(&mut file);
            types::TypeList::from(default)
        }
    }
}

fn default_config() -> Vec<types::Type> {
    vec!(
        types::Type::new("string", ".*"),
        types::Type::new("float", r"[-+]?(?:(?:\d+(?:\.\d*)?)|\.\d+)"),
        types::Type::new("int", r"[-+]?\d+")
    )
}

fn get_config_from_file(config_file: &str) -> Vec<types::Type> {
    let config_file: String = match fs::read_to_string(config_file) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Can not read \"{}\"", config_file);
            process::exit(1);
        }
    };
    let mut list = Vec::new();
    let lines:Vec<&str> = config_file.split('\n').collect();
     
    for line in lines {
        let values:Vec<&str> = line.splitn(2, ' ').collect();
        if values.len() == 2 {
            list.push(types::Type::new(values[0], values[1]));
        }
        
    }
    list
}

fn read_input_from_stdin() -> String{
    let mut input = String::new();
    for line in io::stdin().lock().lines() {
        let line = match line {
            Ok(e) => e,
            Err(_) => continue
        };
        input.push_str(&line[..]);
        input.push('\n');
    }
    input
}

enum ConfigFileType {
    Append(String),
    ReplaceDefault(String),
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_config() {
        assert_eq!(types::TypeList::from(default_config()), get_config(ConfigFileType::None));
    }

    #[test]
    fn get_file_config() {
        let types = types::TypeList::from(vec!(
            types::Type::new("int", r"\d+"),
            types::Type::new("float", r"\d+.\d+"),
            types::Type::new("bool", "[yn]")
        ));
        assert_eq!(types.get_types_vec(), get_config(ConfigFileType::ReplaceDefault(String::from("test_data/config"))).get_types_vec());
    }

    #[test]
    fn get_file_config_merge() {
        let types = types::TypeList::from(vec!(
            types::Type::new("string", ".*"),
            types::Type::new("int", r"\d+"),
            types::Type::new("float", r"\d+.\d+"),
            types::Type::new("bool", "[yn]")
        ));
        assert_eq!(types.get_types_vec(), get_config(ConfigFileType::Append(String::from("test_data/config"))).get_types_vec());
    }
}
