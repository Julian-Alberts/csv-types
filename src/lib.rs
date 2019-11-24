pub mod types;
use csv;
use std::thread;

pub fn get_types(csv: &str, type_list: types::TypeList, has_headers: bool) -> (Vec<String>, Vec<Vec<types::Type>>) {
    let mut csv = parse_csv(csv);

    let headers = if has_headers {
        csv[0].clone()
    } else {
        Vec::new()
    };

    if has_headers {
        csv.remove(0);
    }

    let fliped_csv = flip_vec(&csv);
    let mut join_heandlers = Vec::new();
    for col in fliped_csv {
        let type_list = type_list.get_types_vec().clone();
        join_heandlers.push(thread::spawn(move || {
            types::get_matching_types(&col, &type_list)
        }));
    }
    let mut col_types = Vec::new();
    for handler in join_heandlers {
        let col_type_col = match handler.join() {
            Ok(t) => t,
            Err(_) => vec!(types::Type::new("ERROR could not read type", ""))
        };
        col_types.push(col_type_col);
    }

    return (headers, col_types);
}

pub fn parse_csv(csv: &str) -> Vec<Vec<String>> {
    let mut csv_reader = csv::ReaderBuilder::new().has_headers(false).from_reader(csv.as_bytes());
    let mut csv = Vec::new();
    for record in csv_reader.records() {
        let record: csv::StringRecord = match record {
            Ok(e) => e,
            Err(_) => continue
        };

        let mut row = Vec::new();
        for n in 0..record.len() {
            let column = record[n].to_owned();
            row.push(column);
        }

        csv.push(row);
    };
    return csv;
}

fn flip_vec(vec: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut fliped_vec = Vec::new();
    for row in vec {
        for column_id in 0..row.len() {
            let fliped_column = match fliped_vec.get_mut(column_id) {
                Some(col) => col,
                None => {
                    let column = Vec::new();
                    fliped_vec.push(column);
                    &mut fliped_vec[column_id]
                }
            };
            fliped_column.push(row[column_id].clone());
        }
    }
    return fliped_vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_csv_one_line() {
        let expected = vec!(vec!("v1".to_owned(), "v2".to_owned(), "v3".to_owned()));
        let csv = parse_csv("v1,v2,v3");
        assert_eq!(expected, csv);
    }

    #[test]
    fn parse_csv_multiple_lines() {
        let expected = vec!(vec!("v4".to_owned(), "v34".to_owned(), "v7".to_owned()), vec!("v1".to_owned(), "v2".to_owned(), "v3".to_owned()));
        let csv = parse_csv("v4,v34,v7\nv1,v2,v3");
        assert_eq!(expected, csv);
    }

    #[test]
    fn flip_simple_vec() {
        let orig = vec!(vec!("a1".to_owned(), "b1".to_owned()), vec!("a2".to_owned(), "b2".to_owned()));
        let result = vec!(vec!("a1", "a2"), vec!("b1", "b2"));
        assert_eq!(result, flip_vec(&orig));
    }
}
