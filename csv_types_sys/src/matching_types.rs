use super::types;
use super::vec;
use super::Error;
use std::thread;

pub fn get_matching_types(
    csv: Vec<Vec<String>>,
    type_list: types::TypeList,
    max_threads: usize,
) -> Result<Vec<Vec<types::Type>>, Error> {
    let flipped_csv = vec::flip_vec(&csv);
    let col_sets = vec::split_vec_equal(&flipped_csv, max_threads);

    let col_types = search_types(col_sets, &type_list)?;

    Ok(col_types)
}

fn search_types(
    col_sets: Vec<Vec<Vec<String>>>,
    type_list: &types::TypeList,
) -> Result<Vec<Vec<types::Type>>, Error> {
    let mut join_handlers = Vec::new();
    for col_set in col_sets {
        let type_list = type_list.get_types_vec().clone();
        join_handlers.push(thread::spawn(move || {
            let mut col_types = Vec::new();
            for col in col_set {
                col_types.push(types::get_matching_types(&col, &type_list))
            }
            col_types
        }));
    }
    let mut col_types = Vec::new();
    for handler in join_handlers {
        let col_type_cols = match handler.join() {
            Ok(ctc) => ctc,
            Err(_) => return Err(Error::Join),
        };
        for col_type_col in col_type_cols {
            col_types.push(col_type_col);
        }
    }
    Ok(col_types)
}

#[cfg(test)]
mod tests {

    #[test]
    fn match_only_strings() {
        let types = vec![
            super::types::Type::new("str", ".*"),
            super::types::Type::new("num", r"\d*"),
        ];
        let csv = vec![vec![
            String::from("W"),
            String::from("r"),
            String::from("asd"),
        ]];
        let result =
            super::get_matching_types(csv, super::types::TypeList::from(types), 4).map(|r| {
                r.into_iter()
                    .map(|col| col.into_iter().map(|ty| ty.name).collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            });
        assert_eq!(
            Ok(vec![
                vec!["str".to_owned()],
                vec!["str".to_owned()],
                vec!["str".to_owned()]
            ]),
            result
        );
    }

    #[test]
    fn match_multiple_types() {
        let types = vec![
            super::types::Type::new("str", ".*"),
            super::types::Type::new("num", r"\d*"),
        ];
        let csv = vec![vec![
            String::from("W"),
            String::from("r"),
            String::from("3"),
        ]];
        let result =
            super::get_matching_types(csv, super::types::TypeList::from(types), 4).map(|r| {
                r.into_iter()
                    .map(|col| col.into_iter().map(|ty| ty.name).collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            });
        assert_eq!(
            Ok(vec![
                vec!["str".to_owned()],
                vec!["str".to_owned()],
                vec!["str".to_owned(), "num".to_owned()]
            ]),
            result
        );
    }
}
