use std::io::{BufRead, prelude::*, Result};


pub fn parse_from_reader(reader: &mut dyn BufRead, seperator: char) -> Result<()> {
    let bytes = reader.bytes().filter_map(|v| v.ok()).collect::<Vec<u8>>();
    let rows = get_rows_from_bytes(&bytes);


    Ok(())
}


fn get_rows_from_bytes<'a>(bytes: &'a Vec<u8>) -> Vec<&'a [u8]> {
    let mut row_start = 0;
    
    let mut rows = bytes.iter()
    .enumerate()
    .fold(Vec::new(), |mut rows, (index, value)| {
    if *value == '\n' as u8 {
        rows.push(&bytes[row_start..index]);
        row_start = index + 1;
    }
    rows
    });
    rows.push(&bytes[row_start..]);

    rows
}

fn get_values(row: &[char]) -> Vec<String> {
    let mut value_start = 0_usize;
    let mut values = Vec::new();
    let mut is_quoted = false;
    let mut value_started_with_quoteation_make = false;
    row.iter().enumerate().for_each(|(index, value)| {
        if *value == ',' && !is_quoted{
            let range = if value_started_with_quoteation_make {
                value_start+1..index-1
            } else {
                value_start..index
            };

            values.push(&row[range]);
            value_start = index + 1;
            value_started_with_quoteation_make = false;
        } else if *value == '"' {
            is_quoted = !is_quoted;
            if index == value_start {
                value_started_with_quoteation_make = true;
            }
        }
    });
    let range = if value_started_with_quoteation_make {
        value_start+1..row.len() - 1
    } else {
        value_start..row.len()
    };
    values.push(&row[range]);

    let mut prev_c = 0 as char;

    values.iter().map(|value| {
        value.iter().filter(|c| {
            if **c == '"' && prev_c == '"' {
                prev_c = 0 as char;
                false
            } else {
                prev_c = **c;
                true
            }
        }).collect::<String>()
    }).collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_values_from_row_simple() {
        let input = &"abc,def,ghi,jkl,mno".chars().collect::<Vec<char>>()[..];
        let expected = vec!["abc","def","ghi","jkl","mno"].iter().map(|f| f.to_string()).collect::<Vec<String>>();

        assert_eq!(get_values(input), expected);
    }

    #[test]
    fn get_values_from_row_komplex() {
        let input = &r#""abc""","""def",hij,"kl,m","""n,op""#.chars().collect::<Vec<char>>()[..];
        let expected = vec!["abc\"","\"def","hij","kl,m","\"n,op"].iter().map(|f| f.to_string()).collect::<Vec<String>>();
        assert_eq!(get_values(input), expected);
    }
}