use std::fmt::Display;

mod assert_matching_rows;
mod matching_types;
pub mod types;
mod vec;

pub fn get_types(
    csv: CsvInput,
    type_list: types::TypeList,
    options: Options,
) -> Result<(Vec<String>, Vec<Vec<types::Type>>), Error> {
    let has_headers = options.has_headers;
    let max_threads = if let Some(threads) = options.max_threads {
        if threads < 1 {
            return Err(Error::ThreadCount);
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

pub fn assert_columns_match(
    csv: CsvInput,
    expected_types: Vec<types::Type>,
    options: Options,
) -> Result<Vec<(usize, Vec<usize>)>, Error> {
    let has_headers = options.has_headers;
    let max_threads = if let Some(threads) = options.max_threads {
        if threads < 1 {
            return Err(Error::ThreadCount);
        }
        threads
    } else {
        1
    };

    let mut csv = vec::csv_to_vec(csv);

    if has_headers {
        get_header(&mut csv);
    }

    let failed_assertions =
        assert_matching_rows::assert_matching_rows(csv, &expected_types, max_threads)?;

    Ok(failed_assertions)
}

fn get_header(csv: &mut Vec<Vec<String>>) -> Vec<String> {
    let headers = csv[0].clone();
    csv.remove(0);
    headers
}

pub enum CsvInput<'a> {
    Csv(&'a str),
    Reader(csv::Reader<&'a [u8]>),
}

pub struct Options {
    pub has_headers: bool,
    pub max_threads: Option<usize>,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    Join,
    ThreadCount,
    ColumnCountNotMatching,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ColumnCountNotMatching => write!(f, "Column count not matching"),
            Self::Join => write!(f, "Could not join threads"),
            Self::ThreadCount => write!(f, "Thread smaller then one"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_types_get_types() {
        let type1 = types::Type::new("T1", "^1$");
        let type2 = types::Type::new("T2", "^2$");
        let typed = types::Type::new("Td", r"^\d$");
        let types = types::TypeList::from(vec![type1.clone(), type2.clone(), typed.clone()]);
        let ret = get_types(
            CsvInput::Csv("1,2,2,1\n2,2,1,1\n3,2,2,1"),
            types,
            Options {
                has_headers: false,
                max_threads: Some(1),
            },
        )
        .map(|c| {
            c.1.into_iter()
                .map(|c| c.into_iter().map(|t| t.name).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        });
        let expected = vec![
            vec![typed.name.to_owned()],
            vec![type2.name.to_owned(), typed.name.to_owned()],
            vec![typed.name.to_owned()],
            vec![type1.name.to_owned(), typed.name.to_owned()],
        ];
        assert_eq!(Ok(expected), ret);
    }

    #[test]
    fn get_types_get_types_multi_threads() {
        let type1 = types::Type::new("T1", "^1$");
        let type2 = types::Type::new("T2", "^2$");
        let typed = types::Type::new("Td", r"^\d$");
        let types = types::TypeList::from(vec![type1.clone(), type2.clone(), typed.clone()]);
        let ret = get_types(
            CsvInput::Csv("1,2,2,1\n2,2,1,1\n3,2,2,1"),
            types,
            Options {
                has_headers: false,
                max_threads: Some(2),
            },
        )
        .map(|c| {
            c.1.into_iter()
                .map(|c| c.into_iter().map(|t| t.name).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        });
        let expected = vec![
            vec![typed.name.clone()],
            vec![type2.name.clone(), typed.name.clone()],
            vec![typed.name.clone()],
            vec![type1.name.clone(), typed.name.clone()],
        ];
        assert_eq!(Ok(expected), ret);
    }

    #[test]
    fn get_types_thread_count_error() {
        let type_def = types::Type::new("test", "^.*$");
        let types = types::TypeList::from(vec![type_def.clone()]);
        match get_types(
            CsvInput::Csv(""),
            types,
            Options {
                has_headers: false,
                max_threads: Some(0),
            },
        ) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                Error::ThreadCount => assert!(true),
                _ => assert!(false),
            },
        };
    }

    #[test]
    fn get_header_success() {
        let mut input = vec![
            vec!["h1".to_owned(), "h2".to_owned()],
            vec!["v1".to_owned(), "v2".to_owned()],
        ];
        let h = get_header(&mut input);
        assert_eq!(vec!("h1".to_owned(), "h2".to_owned()), h);
        assert_eq!(vec!(vec!("v1".to_owned(), "v2".to_owned())), input);
    }
}
