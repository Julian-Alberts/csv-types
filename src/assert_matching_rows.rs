use super::types;
use super::Err;
use super::vec;
use std::thread;

pub fn assert_matching_rows(csv: Vec<Vec<String>>, expected_types: &Vec<types::Type>, max_threads: usize) -> Result<Vec<(usize)>, Err> {
    let fliped_csv = vec::flip_vec(&csv);
    
    if fliped_csv.len() != expected_types.len() {
        return Err(Err::ColumnCountNotMatching);
    }

    let col_sets = vec::split_vec_equal(&fliped_csv, max_threads);
    let expected_types = vec::split_vec_equal(expected_types, max_threads);

    let missmatched_rows = check_for_type_match(col_sets, &expected_types);

    return missmatched_rows;
}

fn check_for_type_match(col_sets: Vec<Vec<Vec<String>>>, expected_types: &[Vec<types::Type>]) -> Result<Vec<(usize)>, Err> {
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
                        missmatched_rows.push(row_index);
                    }
                }
                
            }
            return missmatched_rows;
        }));
    }
    let mut missmatched_rows = Vec::new();
    for handler in join_heandlers {
        let col_type_cols = match handler.join() {
            Ok(ctc) => ctc,
            Err(_) => return Err(Err::Join)
        };
        for col_type_col in col_type_cols {
            missmatched_rows.push(col_type_col);
        }
    }
    Ok(missmatched_rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_matching_rows_column_count_not_matching() {
        let csv = vec!(vec!(), vec!());
        let expected_types = vec!(types::Type {name: String::from(""), pattern: String::from("")});
        assert!(if let Err(err) = assert_matching_rows(csv, &expected_types, 1) {
            if let Err::ColumnCountNotMatching = err {
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
            types::Type {name: String::from("str"), pattern: String::from("^.*$")}, 
            types::Type {name: String::from("str"), pattern: String::from("^.*$")}, 
            types::Type {name: String::from("str"), pattern: String::from(r"^\d$")}
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
            vec!(vec!("w".to_owned())),
            vec!(vec!("w".to_owned())),
            vec!(vec!("w".to_owned()))
        );
        
        let expected_types = vec!(
            vec!(types::Type {name: "".to_owned(), pattern: "^.*$".to_owned()}),
            vec!(types::Type {name: "".to_owned(), pattern: "^.*$".to_owned()}),
            vec!(types::Type {name: "".to_owned(), pattern: "^.*$".to_owned()})
        );

        match check_for_type_match(col_sets, &expected_types) {
            Ok(e) => assert_eq!(0, e.len()),
            _ => assert!(false)
        };
    }

    #[test]
    fn check_for_type_match_not_first_line() {
        let col_sets = vec!(
            vec!(vec!("w".to_owned())),
            vec!(vec!("w".to_owned())),
            vec!(vec!("w".to_owned()))
        );
        
        let expected_types = vec!(
            vec!(types::Type {name: "".to_owned(), pattern: r"^\d$".to_owned()}),
            vec!(types::Type {name: "".to_owned(), pattern: "^.*$".to_owned()}),
            vec!(types::Type {name: "".to_owned(), pattern: "^.*$".to_owned()})
        );

        match check_for_type_match(col_sets, &expected_types) {
            Ok(e) => assert_eq!(&[0], &e[..]),
            _ => assert!(false)
        };
    }

    #[test]
    fn check_for_type_match_not_first_second_row() {
        let col_sets = vec!(
            vec!(vec!("2".to_owned(), String::from("w"))),
            vec!(vec!("w".to_owned(), String::from("w"))),
            vec!(vec!("w".to_owned(), String::from("w")))
        );
        
        let expected_types = vec!(
            vec!(types::Type {name: "".to_owned(), pattern: r"^\d$".to_owned()}),
            vec!(types::Type {name: "".to_owned(), pattern: "^.*$".to_owned()}),
            vec!(types::Type {name: "".to_owned(), pattern: "^.*$".to_owned()})
        );

        match check_for_type_match(col_sets, &expected_types) {
            Ok(e) => assert_eq!(&[1], &e[..]),
            _ => assert!(false)
        };
    }
}
