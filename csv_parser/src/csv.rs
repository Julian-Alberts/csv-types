use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;

pub struct CSV {
    csv: HashMap<String, HashMap<TypeId, Vec<Box<dyn Any>>>>
}

impl CSV {

    fn filter_by<T>(&self, column: &str, value: T) where T: 'static {
        let column = self.csv.get(column);

        let column = match column {
            Some(c) => c,
            None => return
        };

        let typed_column = column.get(&TypeId::of::<T>());
        let typed_column = match typed_column {
            Some(tc) => tc,
            None => return
        };

    }

}