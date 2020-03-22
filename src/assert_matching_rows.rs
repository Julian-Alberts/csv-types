use super::types;
use super::Error;
use super::vec;
use std::thread;

pub fn assert_matching_rows(csv: Vec<Vec<String>>, expected_types: &[types::Type], max_threads: usize) -> Result<Vec<(usize, Vec<usize>)>, Error> {
    let fliped_csv = vec::flip_vec(&csv);
    
    if fliped_csv.len() != expected_types.len() {
        return Err(Error::ColumnCountNotMatching);
    }

    let col_sets = vec::split_vec_equal(&fliped_csv, max_threads);
    let expected_types = vec::split_vec_equal(expected_types, max_threads);

    check_for_type_match(col_sets, &expected_types)
}

fn check_for_type_match(col_sets: Vec<Vec<Vec<String>>>, expected_types: &[Vec<types::Type>]) -> Result<Vec<(usize, Vec<usize>)>, Error> {
    let mut join_heandlers = Vec::new();
    assert!(col_sets.len() == expected_types.len());

    for (col_set_index, col_set) in col_sets.iter().enumerate() {
        let col_set = col_set.clone();
        let expected_types = expected_types[col_set_index].clone();

        join_heandlers.push(thread::spawn(move || {
            let mut missmatched_rows = Vec::new(); 
            for (col_index, col) in col_set.iter().enumerate() {
                for (row_index, value) in col.iter().enumerate() {
                    let type_def = &expected_types[col_index];

                    if !types::check_if_type_matches(value, type_def) {
                        let col = if col_set_index == 0 {
                            col_index
                        } else {
                            col_index * col_set_index + col_set_index
                        };
                        missmatched_rows.push((row_index, col));
                    }
                }
                
            }
            missmatched_rows
        }));
    }
    let mut missmatched_rows = Vec::new();
    for handler in join_heandlers {
        let col_type_cols = match handler.join() {
            Ok(ctc) => ctc,
            Err(_) => return Err(Error::Join)
        };
        for col_type_col in col_type_cols {
            missmatched_rows.push(col_type_col);
        }
    }

    let mut failed_assertions_joined = Vec::new();
    let mut failed_assertion_index = 0;
    let mut failed_assertion;

    while failed_assertion_index < missmatched_rows.len() {
        failed_assertion = match missmatched_rows.get(failed_assertion_index) {
            Some(a) => a,
            None => continue
        };
        let row = failed_assertion.0;
        let mut failed_assertion_list = (row, Vec::new());
        while row == failed_assertion.0 {
            failed_assertion_list.1.push(failed_assertion.1);

            failed_assertion_index += 1;
            failed_assertion = match missmatched_rows.get(failed_assertion_index) {
                Some(a) => a,
                None => break
            }
        }
        failed_assertions_joined.push(failed_assertion_list);
    }

    Ok(failed_assertions_joined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_matching_rows_column_count_not_matching() {
        let csv = vec!(vec!(), vec!());
        let expected_types = vec!(types::Type {name: String::from(""), pattern: String::from("")});
        assert!(if let Err(err) = assert_matching_rows(csv, &expected_types, 1) {
            if let Error::ColumnCountNotMatching = err {
                true
            } else {
                false
            }
        } else {
            false
        });
    }

    #[test]
    fn assert_matching_rows_all_matching() {
        let csv = vec!(vec!(String::from("w"),String::from("w"),String::from("1")), vec!(String::from("w"),String::from("w"),String::from("2")));
        let expected_types = vec!(
            types::Type::new("str", "^.*$"), 
            types::Type::new("str", "^.*$"), 
            types::Type::new("str", r"^\d$")
        );
        
        if let Ok(val) = assert_matching_rows(csv, &expected_types, 1) {
            assert_eq!(0, val.len());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn check_for_type_match_all_lines_match() {
        let col_sets = vec!(
            vec!(vec!(String::from("w"))),
            vec!(vec!(String::from("w"))),
            vec!(vec!(String::from("w")))
        );
        
        let expected_types = vec!(
            vec!(types::Type::new("", ".*")),
            vec!(types::Type::new("", ".*")),
            vec!(types::Type::new("", ".*"))
        );

        match check_for_type_match(col_sets, &expected_types) {
            Ok(e) => assert_eq!(0, e.len()),
            _ => assert!(false)
        };
    }

    #[test]
    fn check_for_type_match_not_first_line() {
        let col_sets = vec!(
            vec!(vec!(String::from("w"))),
            vec!(vec!(String::from("w"))),
            vec!(vec!(String::from("w")))
        );
        
        let expected_types = vec!(
            vec!(types::Type::new("", r"\d")),
            vec!(types::Type::new("", ".*")),
            vec!(types::Type::new("", ".*")),
        );

        match check_for_type_match(col_sets, &expected_types) {
            Ok(e) => assert_eq!(&[(0, vec!(0))], &e[..]),
            _ => assert!(false)
        };
    }

    #[test]
    fn check_for_type_match_not_first_second_row() {
        let col_sets = vec![
            vec!(vec![String::from("2"), String::from("w")]),
            vec!(vec!(String::from("w"), String::from("w"))),
            vec!(vec!(String::from("w"), String::from("w")))
        ];
        
        let expected_types = vec!(
            vec!(types::Type::new("", r"\d")),
            vec!(types::Type::new("", ".*")),
            vec!(types::Type::new("", ".*")),
        );

        match check_for_type_match(col_sets, &expected_types) {
            Ok(e) => assert_eq!(&[(1, vec!(0))], &e[..]),
            _ => assert!(false)
        };
    }

    #[test]
    fn test() {
        let col_sets = vec!(
            vec!(vec!(String::from("w"), String::from("w"))),
            vec!(vec!(String::from("2"), String::from("w"))),
            vec!(vec!(String::from("w"), String::from("2")))
        );

        let expected_types = vec!(
            vec!(types::Type::new("string", ".*")),
            vec!(types::Type::new("int", r"\d*")),
            vec!(types::Type::new("int", r"\d*"))
        );

        match check_for_type_match(col_sets, &expected_types) {
            Ok(e) => {
                assert!(e.contains(&(0, vec!(2))));
                assert!(e.contains(&(1, vec!(1))));
            },
            _ => assert!(false)
        };
    }
}
