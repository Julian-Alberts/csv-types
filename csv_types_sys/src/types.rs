use regex::Regex;
use std::collections::HashMap;

type TypesMap = HashMap<String, Type>;
type TypeVec = Vec<Type>;

#[derive(Debug, Clone)]
pub struct Type {
    pub pattern: Regex,
    pub name: String,
}

impl Type {
    pub fn new(name: &str, pattern: &str) -> Self {
        Self {
            name: name.to_owned(),
            pattern: Regex::new(&format!("^{}$", pattern)).unwrap(),
        }
    }
}

pub fn get_matching_types(column: &[String], type_list: &[Type]) -> TypeVec {
    let mut type_list = type_list.to_owned();
    type_list.retain(|type_def| {
        let reg = &type_def.pattern;
        for value in column.iter() {
            if !reg.is_match(value) {
                return false;
            }
        }
        true
    });
    type_list
}

pub fn check_if_type_matches(value: &str, type_def: &Type) -> bool {
    let reg = &type_def.pattern;
    reg.is_match(value)
}

#[derive(Debug, Default)]
pub struct TypeList {
    map: TypesMap,
    list: TypeVec,
}

impl TypeList {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            list: Vec::new(),
        }
    }

    pub fn from(list: TypeVec) -> Self {
        let mut instance = Self::new();
        for item in list {
            instance.add_type(item);
        }
        instance
    }

    pub fn add_type(&mut self, type_config: Type) {
        if self.map.get(&type_config.name).is_none() {
            self.list.push(type_config.clone());
        } else {
            self.list.retain(|t| t.name != type_config.name);
            self.list.push(type_config.clone());
        }
        self.map.insert(type_config.name.clone(), type_config);
    }

    pub fn get_types_vec(&self) -> &TypeVec {
        &self.list
    }

    pub fn get_types_map(&self) -> &TypesMap {
        &self.map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_strings() {
        let str1 = "str1".to_owned();
        let str2 = "str2".to_owned();
        let str3 = "str3".to_owned();
        let col = vec![str1, str2, str3];
        let mut tl = TypeList::new();
        tl.add_type(Type::new("string", ".*"));
        let expected = vec!["string".to_owned()];
        assert_eq!(expected, get_matching_types(&col, &tl.get_types_vec()).into_iter().map(|a| a.name.to_owned()).collect::<Vec<_>>());
    }

    #[test]
    fn match_multiple_types() {
        let str1 = "1".to_owned();
        let str2 = "2".to_owned();
        let str3 = "343".to_owned();
        let col = vec![str1, str2, str3];
        let mut tl = TypeList::new();
        tl.add_type(Type::new("string", ".*"));
        tl.add_type(Type::new("int", r"^\d*$"));
        let expected = vec!["string".to_owned(), "int".to_owned()];
        assert_eq!(expected, get_matching_types(&col, &tl.get_types_vec()).into_iter().map(|a| a.name.to_owned()).collect::<Vec<_>>());
    }

    #[test]
    fn match_multiple_types_one_not_matching() {
        let str1 = "1".to_owned();
        let str2 = "w2".to_owned();
        let str3 = "343".to_owned();
        let col = vec![str1, str2, str3];
        let mut tl = TypeList::new();
        tl.add_type(Type::new("string", ".*"));
        tl.add_type(Type::new("int", r"^\d*$"));
        let expected = vec!["string".to_owned()];
        assert_eq!(expected, get_matching_types(&col, &tl.get_types_vec()).into_iter().map(|a| a.name.to_owned()).collect::<Vec<_>>());
    }
}
