pub mod assert_types;
pub mod matching_types;

pub trait SubCommand {
    fn get_command(&self) -> &'static str;
    fn run(&self, args: Vec<String>);
}

