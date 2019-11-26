use std::{io, io::prelude::*};
use csvtypes;
use csvtypes::types;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::fs;
use std::process;

fn main() {
    let mut print_to_table = false;
    let mut config_file = String::new();
    let mut header = false;
    setup_args(&mut print_to_table, &mut config_file, &mut header);

    let config = get_config(&config_file);
    let type_list = types::TypeList::from(config);

    let input = read_input_from_stdin();
    let (mut headers, mut types) = match csvtypes::get_types(&input[..], type_list, header) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Could not determin types");
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

fn setup_args(print_to_table: &mut bool, config_file: &mut String, header: &mut bool) {
    let mut ap = ArgumentParser::new();
    ap.refer(print_to_table)
    .add_option(&["-h", "--human-readable"], StoreTrue, "print in table");
    ap.refer(header)
    .add_option(&["--header"], StoreTrue, "File has header");
    ap.refer(config_file)
    .add_option(&["-c", "--config-file"], Store, "custom config file path");
    ap.parse_args_or_exit();
}

fn get_config(config_file: &str) -> Vec<types::Type> {
    if config_file.is_empty() {
        default_config()
    } else {
        let config_file: String = match fs::read_to_string(config_file) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Can not read \"{}\"", config_file);
                process::exit(1);
            }
        };
        let mut list = Vec::new();
        let mut lines:Vec<&str> = config_file.split("\n").collect();
         
        for (index, line) in lines.iter_mut().enumerate() {
            let values:Vec<&str> = line.splitn(2, " ").collect();
            if values.len() != 2 {
                eprintln!("Error in config file in line \"{}\"", index);
                process::exit(1);
            }
            list.push(types::Type::new(values[0], values[1]));
        }
        return list;
    }
}

fn default_config() -> Vec<types::Type> {
    vec!(
        types::Type::new("string", "^.*$"),
        types::Type::new("float", r"^[-+]?(?:(?:\d+(?:\.\d*)?)|\.\d+)$"),
        types::Type::new("int", r"^[-+]?\d+$")
    )
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
