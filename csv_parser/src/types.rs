pub enum ValueType {
    String(String),
    Number(f64)
}

impl ValueType {

    pub fn get_types(value: &str) -> Vec<ValueType> {
        let mut types = Vec::new();
        types.push(to_string(value));
        types.push(to_number(value));
        
        types.into_iter().filter_map(|f| f).collect::<Vec<ValueType>>()
    }

}

fn to_string(value: &str) -> Option<ValueType> {
    Some(ValueType::String(value.to_owned()))
}

fn to_number(value: &str) -> Option<ValueType> {
    let value = value.parse::<f64>().ok()?;
    Some(ValueType::Number(value))
}

pub struct NumberValue {
    value: f64
}

impl PartialEq for NumberValue {

    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }

}

impl std::cmp::PartialOrd for NumberValue {

    fn ge(&self, other: &Self) -> bool {
        self.value.ge(&other.value)
    }

    fn gt(&self, other: &Self) -> bool {
        self.value.gt(&other.value)
    }

    fn le(&self, other: &Self) -> bool {
        self.value.le(&other.value)
    }

    fn lt(&self, other: &Self) -> bool {
        self.value.lt(&other.value)
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }

}