use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;

pub struct CSV {
    csv: HashMap<String, Column>
}

impl CSV {

    fn get_header(&self) -> std::collections::hash_map::Keys<String, Column>{
        self.csv.keys()
    }

    fn get_column(&self, column: &str) -> Option<&Column> {
        match self.csv.get(column) {
            Some(col) => Some(&col),
            None => None
        }
    }

}

pub struct Column {
    column: HashMap<TypeId, Vec<Box<dyn Any>>>
}

impl Column {

    pub fn get_values<'a, T>(&'a self) -> Option<Vec<&'a T>> where T: 'static{
        let col = self.column.get(&TypeId::of::<T>())?;

        // downcase_ref should have been checked before creation
        Some(col.iter().map(|cell| cell.downcast_ref::<T>().unwrap()).collect::<Vec<&T>>())
    }

}