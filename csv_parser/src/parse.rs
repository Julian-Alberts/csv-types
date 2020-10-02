use std::io::{BufRead, prelude::*, Result};
use crate::types::ValueType;


pub fn parse_from_reader(reader: &mut dyn BufRead, seperator: char) -> Result<Vec<Vec<String>>> {
    let chars = reader
    .bytes()
    .filter_map(|v| v.ok())
    .map(|v| v as char)
    .collect::<Vec<char>>();
    
    let rows = get_rows(&chars[..]);
    
    let csv_values = rows
    .iter()
    .map(|row| get_values(row, seperator))
    .collect::<Vec<Vec<String>>>();

    Ok(csv_values)
}

pub fn typed(csv: &[Vec<String>]) -> Vec<Vec<Vec<ValueType>>> {
    csv.iter().map(|row| {
        row.iter().map(|cell| ValueType::get_types(&cell[..])).collect::<Vec<Vec<ValueType>>>()
    }).collect::<Vec<Vec<Vec<ValueType>>>>()
}

fn get_rows(data: &[char]) -> Vec<&[char]> {
    let mut row_start = 0_usize;
    let mut rows = Vec::new();
    let mut is_quoted = false;
    data.iter().enumerate().for_each(|(index, value)| {
        if *value == '\n' && !is_quoted {
            rows.push(&data[row_start..index]);
            row_start = index + 1;
        } else if *value == '"' {
            is_quoted = !is_quoted;
        }
    });

    rows.push(&data[row_start..]);
    rows
}

fn get_values(row: &[char], seperator: char) -> Vec<String> {
    let mut value_start = 0_usize;
    let mut values = Vec::new();
    let mut is_quoted = false;
    let mut value_started_with_quoteation_make = false;
    row.iter().enumerate().for_each(|(index, value)| {
        if *value == seperator && !is_quoted{
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

        assert_eq!(get_values(input, ','), expected);
    }

    #[test]
    fn get_values_from_row_complex() {
        let input = &r#""abc""","""def",hij,"kl,m","""n,op""#.chars().collect::<Vec<char>>()[..];
        let expected = vec!["abc\"","\"def","hij","kl,m","\"n,op"].iter().map(|f| f.to_string()).collect::<Vec<String>>();
        assert_eq!(get_values(input, ','), expected);
    }

    #[test]
    fn split_rows_simple() {
        let input = &"abc,def\nghi,jkl\nmno,pqr".chars().collect::<Vec<char>>()[..];
        let expected = vec!["abc,def","ghi,jkl","mno,pqr"].iter().map(|f| f.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let expected = expected.iter().map(|f| &f[..]).collect::<Vec<&[char]>>();

        assert_eq!(get_rows(input), expected);
    }

    #[test]
    fn split_rows_complex() {
        let input = &"\"a\nbc\",def\nghi,jkl\nmno,pqr".chars().collect::<Vec<char>>()[..];
        let expected = vec!["\"a\nbc\",def","ghi,jkl","mno,pqr"].iter().map(|f| f.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let expected = expected.iter().map(|f| &f[..]).collect::<Vec<&[char]>>();

        assert_eq!(get_rows(input), expected);
    }
}