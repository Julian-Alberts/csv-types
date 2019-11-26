pub mod types;
use csv;
use std::thread;

pub fn get_types(csv: &str, type_list: types::TypeList, has_headers: bool) -> Result<(Vec<String>, Vec<Vec<types::Type>>), Box<dyn std::any::Any + std::marker::Send>> {
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
    let col_sets = split_vec_equal(fliped_csv, 3);//TODO Add max_thread

    let mut join_heandlers = Vec::new();
    for col_set in col_sets {
        let type_list = type_list.get_types_vec().clone();
        let col_set = col_set.clone();
        join_heandlers.push(thread::spawn(move || {
            let mut col_types = Vec::new(); 
            for col in col_set {
                col_types.push(types::get_matching_types(&col, &type_list))
            }
            return col_types;
        }));
    }
    let mut col_types = Vec::new();
    for handler in join_heandlers {
        let col_type_cols = handler.join()?;
        for col_type_col in col_type_cols {
            col_types.push(col_type_col);
        }
    }

    return Ok((headers, col_types));
}

fn split_vec_equal<T: Clone>(vec: Vec<T>, max_threads: usize) -> Vec<Vec<T>> {
    let mut max_threads = max_threads;
    if max_threads == 0 {
        max_threads = 1;
    }

    let mut splited = Vec::new();
    let mut end = 0;
    
    for i in 0..max_threads {
        let start = end;
        end += vec.len() / max_threads;
        if vec.len() % max_threads > i {
            end += 1;
        }

        splited.push(Vec::from(&vec[start..end]));
    }
    return splited;
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

    #[test]
    fn split_vec_equal_matching_threads() {
        let orig_vec = vec!("a", "b", "c", "d");

        let result = split_vec_equal(orig_vec, 4);
        let expected = vec!(vec!("a"), vec!("b"), vec!("c"), vec!("d"));
        assert_eq!(expected, result)
    }

    #[test]
    fn split_vec_equal_multiple_per_thread() {
        let orig_vec = vec!("a", "b", "c", "d");

        let result = split_vec_equal(orig_vec, 2);
        let expected = vec!(vec!("a", "b"), vec!("c", "d"));
        assert_eq!(expected, result);
    }

    #[test]
    fn split_vec_equal_count_not_matching() {
        let orig_vec = vec!("a", "b", "c");

        let result = split_vec_equal(orig_vec, 2);
        let expected = vec!(vec!("a", "b"), vec!("c"));
        assert_eq!(expected, result)
    }

    #[test]
    fn split_vec_equal_more_threads() {
        let orig_vec = vec!("a", "b", "c");

        let result = split_vec_equal(orig_vec, 4);
        let expected = vec!(vec!("a"), vec!("b"), vec!("c"), vec!());
        assert_eq!(expected, result)
    }
}
