pub struct AssertTypes;
use csv_types_sys;
use csv_types_sys::types;
use crate::print_result;
use std::process;
use crate::config::ConfigFileType;
use argparse::{ArgumentParser, StoreTrue, StoreOption, Store};



impl AssertTypes {
    fn assert_types(csv: &str, type_list: types::TypeList, options: csv_types_sys::Options, asserted_types: String, machine_readable: bool) {
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
        
        let rows = match csv_types_sys::assert_columns_match(csv_types_sys::CsvInput::Csv(csv), expected_types, options) {
            Ok(rows) => rows,
            Err(err) => {
                match err {
                    csv_types_sys::Error::Join => eprintln!("Could not join threads."),
                    csv_types_sys::Error::ThreadCount => eprintln!("The thread count must be bigger than 0"),
                    csv_types_sys::Error::ColumnCountNotMatching => eprintln!("The given number of types does not match the number of columns"),
                }
                process::exit(1);
            }
        };
    
        print_result::assert_types(&rows[..], machine_readable);
    
    }

    fn setup_args(args: Vec<String>) -> (ConfigFileType, String, csv_types_sys::Options, bool) {
        let mut config_file_replace_default = String::new();
        let mut config_file = String::new();
        let mut options =  csv_types_sys::Options {
            has_headers: false,
            max_threads: None
        };
        let mut assert = String::new();
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
        ap.refer(&mut machine_readable)
        .add_option(&["-m"], StoreTrue, "Machine readable format");
        ap.refer(&mut assert)
        .add_argument("Expected types", Store, "Expected types comma seperated");
        
        
        ap.parse(args, &mut std::io::stdout(), &mut std::io::stderr())
            .map_err(|c| std::process::exit(c))
            .ok();
        
        
        
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
}

impl super::SubCommand for AssertTypes {

    fn get_command(&self) -> &'static str {
        "assert"
    }

    fn run(&self, args: Vec<String>) {
        let (config_file, asserted_types, options, machine_readable) = Self::setup_args(args);
        let csv = crate::read_input_from_stdin();
        let type_list = crate::config::get_config(config_file);
        Self::assert_types(&csv, type_list, options, asserted_types, machine_readable);
    }

}