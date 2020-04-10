use csv_types_sys::types;
use std::fs;
use std::process::exit;

pub fn get_config(config_file: ConfigFileType) -> types::TypeList {
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
            exit(1);
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

pub enum ConfigFileType {
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