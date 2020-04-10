use csv_types_sys;
use csv_types_sys::types;
use crate::print_result;
use std::process;
use crate::ConfigFileType;
use argparse::{ArgumentParser, StoreTrue, StoreOption, Store};

pub struct MatchingTypes;

impl MatchingTypes {
    fn matching_types(input: &str, type_list: types::TypeList, options: csv_types_sys::Options, machine_readable: bool) {
        let (headers, types) = match csv_types_sys::get_types(input, type_list, options) {
            Ok(r) => r,
            Err(err) => {
                match err {
                    csv_types_sys::Err::Join => eprintln!("Could not join threads."),
                    csv_types_sys::Err::ThreadCount => eprintln!("The thread count needs to be bigger than 0"),
                    _ => eprintln!("An unknown Error accoured")
                }
                process::exit(1);
            }
        };
    
        print_result::matching_types(&types, &headers, machine_readable);
    }

    fn setup_args(args: Vec<String>) -> (ConfigFileType, csv_types_sys::Options, bool) {
        let mut config_file_replace_default = String::new();
        let mut config_file = String::new();
        let mut options =  csv_types_sys::Options {
            has_headers: false,
            max_threads: None
        };

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
    
        (config_file, options, machine_readable)
    }
}

impl super::SubCommand for MatchingTypes {

    fn get_command(&self) -> &'static str {
        "match"
    }

    fn run(&self, args: Vec<String>) {
        let (config_file, options, machine_readable) = Self::setup_args(args);
        let csv = crate::read_input_from_stdin();
        let type_list = crate::get_config(config_file);
        Self::matching_types(&csv, type_list, options, machine_readable);
    }

}