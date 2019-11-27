use std::{io, io::prelude::*};
use csvtypes;
use csvtypes::types;
use argparse::{ArgumentParser, StoreTrue, StoreOption, Store};
use std::fs;
use std::process;

fn main() {
    let (print_to_table, config_file, options) = setup_args();

    let type_list = get_config(config_file);

    let input = read_input_from_stdin();
    let (mut headers, mut types) = match csvtypes::get_types(&input[..], type_list, options) {
        Ok(r) => r,
        Err(err) => {
            match err {
                csvtypes::Err::Join => {
                    eprintln!("Could not join threads.");
                },
                csvtypes::Err::ThreadCount => {
                    eprintln!("The thread count needs to be bigger than 0")
                }
            }
            process::exit(1);
        }
    };
    display_types(&mut types, &mut headers);
}

fn display_types(types: &mut Vec<Vec<types::Type>>, headers: &mut Vec<String>) {
    let mut width = Vec::new();
    let mut count = Vec::new();
    let mut max_rows = 0; 
    for t1 in types.iter() {
        let mut w = 0;
        if max_rows < t1.len() {
            max_rows = t1.len();
        }
        count.push(t1.len());
        for t in t1 {
            if w < t.name.len() {
                w = t.name.len();
            }
        }
        width.push(w);
    }

    for (index, header) in headers.iter_mut().enumerate() {
        let max_width = width[index];
        if max_width < header.len() {
            width[index] = header.len();
        }
    }

    if headers.len() > 0 {
        let mut complete_width = 0;
        for (col_id, header) in headers.iter_mut().enumerate() {
            let col_width = match width.get(col_id) {
                Some(w) => w,
                None => &(10 as usize)
            };
            print!("| {name:>width$} ", width=col_width, name=header);
            complete_width += 3 + col_width;
        }
        println!("|");

        println!("{:=>width$}", "", width=complete_width+1);
    }

    for row in 0..max_rows {
        for (col_id, col) in types.iter_mut().enumerate() {
            let col_width = match width.get(col_id) {
                Some(w) => w,
                None => &(10 as usize)
            };
            let name = match col.get(row) {
                Some(t) => &t.name,
                None => ""
            };
            print!("| {name:>width$} ", width=col_width, name=name);
        }
        println!("|");
    }
}

fn setup_args() -> (bool, ConfigFileType, csvtypes::Options) {
    let mut config_file_replace_default = String::new();
    let mut print_to_table = false;
    let mut config_file = String::new();
    let mut options =  csvtypes::Options {
        has_headers: false,
        max_threads: None
    };

    let mut ap = ArgumentParser::new();
    ap.refer(&mut print_to_table)
    .add_option(&["-h", "--human-readable"], StoreTrue, "print in table");
    ap.refer(&mut options.has_headers)
    .add_option(&["--header"], StoreTrue, "File has header");
    ap.refer(&mut config_file)
    .add_option(&["-c", "--config-file"], Store, "custom config file path");
    ap.refer(&mut config_file_replace_default)
    .add_option(&["-C", "--config-file-replace-default"], Store, "custom config file path replace default config");
    ap.refer(&mut options.max_threads)
    .add_option(&["--max-threads"], StoreOption, "");
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

    return (print_to_table, config_file, options);
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
        types::Type::new("string", "^.*$"),
        types::Type::new("float", r"^[-+]?(?:(?:\d+(?:\.\d*)?)|\.\d+)$"),
        types::Type::new("int", r"^[-+]?\d+$")
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
    let lines:Vec<&str> = config_file.split("\n").collect();
     
    for line in lines {
        let values:Vec<&str> = line.splitn(2, " ").collect();
        if values.len() == 2 {
            list.push(types::Type::new(values[0], values[1]));
        }
        
    }
    return list;
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
    return input;
}

enum ConfigFileType {
    Append(String),
    ReplaceDefault(String),
    None
}
