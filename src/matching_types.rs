use super::types;
use super::Err;
use super::vec;
use std::thread;

pub fn get_matching_types(csv: Vec<Vec<String>>, type_list: types::TypeList, max_threads: usize)  -> Result<Vec<Vec<types::Type>>, Err> {  

    let fliped_csv = vec::flip_vec(&csv);
    let col_sets = vec::split_vec_equal(&fliped_csv, max_threads);

    let col_types = search_types(col_sets, &type_list)?;

    return Ok(col_types);
}

fn search_types(col_sets: Vec<Vec<Vec<String>>>, type_list: &types::TypeList) -> Result<Vec<Vec<types::Type>>, Err> {
    let mut join_heandlers = Vec::new();
    for col_set in col_sets {
        let type_list = type_list.get_types_vec().clone();
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
        let col_type_cols = match handler.join() {
            Ok(ctc) => ctc,
            Err(_) => return Err(Err::Join)
        };
        for col_type_col in col_type_cols {
            col_types.push(col_type_col);
        }
    }
    Ok(col_types)
}
