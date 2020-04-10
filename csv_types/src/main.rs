use std::{io, io::prelude::*};
use csv_types_sys;
use csv_types_sys::types;
use std::fs;
use std::process;
mod print_result;
mod sub_commands;

fn main() {
    let sub_commands:[Box<dyn sub_commands::SubCommand>; 2] = [
        Box::new(sub_commands::assert_types::AssertTypes {}),
        Box::new(sub_commands::matching_types::MatchingTypes {})
    ];

    let args: Vec<String> = std::env::args().collect();
    let sub_command = match args.get(1) {
        Some(s) => s,
        None => {
            eprintln!("failed");
            std::process::exit(1);
        }
    };

    for command in &sub_commands {
        if command.get_command() == sub_command {
            command.run(Vec::from(&args[1..]));
            break;
        }
    }

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
