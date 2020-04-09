pub mod types;
mod matching_types;
mod assert_matching_rows;
mod vec;

pub fn get_types(csv: &str, type_list: types::TypeList, options: Options) -> Result<(Vec<String>, Vec<Vec<types::Type>>), Err> {
    
    let has_headers = options.has_headers;
    let max_threads = if let Some(threads) = options.max_threads {
        if threads < 1 {
            return Err(Err::ThreadCount);
        }
        threads
    } else {
        1
    };
    
    let mut csv = vec::csv_to_vec(csv);

    let headers = if has_headers {
        get_header(&mut csv)
    } else {
        Vec::new()
    };

    let types = matching_types::get_matching_types(csv, type_list, max_threads)?;

    Ok((headers, types))
}

pub fn assert_columns_match(csv: &str, expected_types: Vec<types::Type>, options: Options) -> Result<Vec<(usize, Vec<usize>)>, Err> {
    let has_headers = options.has_headers;
    let max_threads = if let Some(threads) = options.max_threads {
        if threads < 1 {
            return Err(Err::ThreadCount);
        }
        threads
    } else {
        1
    };
    
    let mut csv = vec::csv_to_vec(csv);

    if has_headers {
        get_header(&mut csv);
    }

    let failed_assertions = assert_matching_rows::assert_matching_rows(csv, &expected_types, max_threads)?;

    Ok(failed_assertions)
}

fn get_header(csv: &mut Vec<Vec<String>>) -> Vec<String> {
    let headers = csv[0].clone();
    csv.remove(0);
    headers
}

pub struct Options {
    pub has_headers: bool,
    pub max_threads: Option<usize>
}

#[derive(PartialEq, Debug)]
pub enum Err {
    Join,
    ThreadCount,
    ColumnCountNotMatching
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_types_get_types() {
        let type1 = types::Type::new("T1", "^1$");
        let type2 = types::Type::new("T2", "^2$");
        let typed = types::Type::new("Td", r"^\d$");
        let types = types::TypeList::from(vec!(type1.clone(), type2.clone(), typed.clone()));
        let ret = match get_types(
            "1,2,2,1\n2,2,1,1\n3,2,2,1", types, Options {has_headers: false,max_threads: Some(1)}) {
            Ok(e) => e,
            Err(_) => {
                assert!(false);
                return;
            }
        };
        let expected = (
            vec!(), 
            vec!(
                vec!(typed.clone()),
                vec!(type2.clone(), typed.clone()),
                vec!(typed.clone()),
                vec!(type1.clone(), typed.clone())
            )
        );
        assert_eq!(expected, ret);
    }

    #[test]
    fn get_types_get_types_multi_threads() {
        let type1 = types::Type::new("T1", "^1$");
        let type2 = types::Type::new("T2", "^2$");
        let typed = types::Type::new("Td", r"^\d$");
        let types = types::TypeList::from(vec!(type1.clone(), type2.clone(), typed.clone()));
        let ret = match get_types(
            "1,2,2,1\n2,2,1,1\n3,2,2,1", types, Options {has_headers: false,max_threads: Some(2)}) {
            Ok(e) => e,
            Err(_) => {
                assert!(false);
                return;
            }
        };
        let expected = (
            vec!(), 
            vec!(
                vec!(typed.clone()),
                vec!(type2.clone(), typed.clone()),
                vec!(typed.clone()),
                vec!(type1.clone(), typed.clone())
            )
        );
        assert_eq!(expected, ret);
    }
    
    #[test]
    fn get_types_thread_count_error() {
        let type_def = types::Type::new("test", "^.*$");
        let types = types::TypeList::from(vec!(type_def.clone()));
        match get_types("", types, Options {
            has_headers: false,
            max_threads: Some(0)
        }) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                Err::ThreadCount => assert!(true),
                _ => assert!(false)
            }
        };
    }

    #[test]
    fn get_header_success() {
        let mut input = vec!(vec!("h1".to_owned(), "h2".to_owned()), vec!("v1".to_owned(), "v2".to_owned()));
        let h = get_header(&mut input);
        assert_eq!(vec!("h1".to_owned(), "h2".to_owned()), h);
        assert_eq!(vec!(vec!("v1".to_owned(), "v2".to_owned())), input);
    }

}
