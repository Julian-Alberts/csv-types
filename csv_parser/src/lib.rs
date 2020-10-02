mod parse;
mod types;
mod csv;

pub use parse::parse_from_reader;
use std::collections::HashMap;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
