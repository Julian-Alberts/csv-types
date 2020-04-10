use std::{io, io::prelude::*};
mod print_result;
mod sub_commands;
mod config;

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
            let mut command_args = args;
            command_args.remove(1);
            command.run(command_args);
            break;
        }
    }

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
