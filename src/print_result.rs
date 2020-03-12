use csvtypes::types;

pub fn matching_types(types: &[Vec<types::Type>], headers: &[String], machine_readable: bool) {
    if machine_readable {
        matching_types_machine_readable(types);
    } else {
        matching_types_human_readable(types, headers);
    }
}

fn matching_types_human_readable(types: &[Vec<types::Type>], headers: &[String]) {
    let mut width = Vec::new();
    let mut count = Vec::new();
    let mut max_rows = 0; 
    for t1 in types.iter() {
        let mut w = 0;
        if max_rows < t1.len() {
            max_rows = t1.len();
        }
        count.push(t1.len());
        for t in t1 {
            if w < t.name.len() {
                w = t.name.len();
            }
        }
        width.push(w);
    }

    for (index, header) in headers.iter().enumerate() {
        let max_width = width[index];
        if max_width < header.len() {
            width[index] = header.len();
        }
    }

    if !headers.is_empty() {
        let mut complete_width = 0;
        for (col_id, header) in headers.iter().enumerate() {
            let col_width = match width.get(col_id) {
                Some(w) => w,
                None => &(10 as usize)
            };
            print!("| {name:>width$} ", width=col_width, name=header);
            complete_width += 3 + col_width;
        }
        println!("|");

        println!("{:=>width$}", "", width=complete_width+1);
    }

    for row in 0..max_rows {
        for (col_id, col) in types.iter().enumerate() {
            let col_width = match width.get(col_id) {
                Some(w) => w,
                None => &(10 as usize)
            };
            let name = match col.get(row) {
                Some(t) => &t.name,
                None => ""
            };
            print!("| {name:>width$} ", width=col_width, name=name);
        }
        println!("|");
    }
}

fn matching_types_machine_readable(types: &[Vec<types::Type>]) {
    for row in types {
        for t in row {
            print!("{},", t.name);
        }
        println!();
    }
}

pub fn assert_types(rows: &[(usize, Vec<usize>)], machine_readable: bool) {
    if machine_readable {
        assert_types_machine_readable(rows)
    } else {
        assert_types_human_readable(rows);
    }
}

fn assert_types_human_readable(rows:&[(usize, Vec<usize>)]) {
    if !rows.is_empty() {
        eprintln!("These rows did not match: ");
        for failed_assertions in rows {
            print!("{}", failed_assertions.0);
            for col in &failed_assertions.1 {
                print!(":{}", col);
            }
            println!();
        }
    } else {
        eprintln!("All rows matched");
    }
}

fn assert_types_machine_readable(rows:&[(usize, Vec<usize>)]) {
    if !rows.is_empty() {
        for failed_assertions in rows {
            print!("{}", failed_assertions.0);
            for col in &failed_assertions.1 {
                print!(":{}", col);
            }
            println!();
        }
    }
}
