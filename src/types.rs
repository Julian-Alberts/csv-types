use std::collections::HashMap;
use regex::Regex;

type TypesMap = HashMap<String, Type>;
type TypeVec = Vec<Type>;

#[derive(PartialEq, Debug, Clone)]
pub struct Type {
    pub pattern: String,
    pub name: String
}

impl Type {
    pub fn new(name: &str, pattern: &str) -> Self {
        Self {
            name: name.to_owned(),
            pattern: format!("^{}$", pattern)
        }
    }
}

pub fn get_matching_types(column: &[String], type_list: &TypeVec) -> TypeVec {
    let mut type_list = type_list.clone();
    type_list.retain(|type_def| {
        let reg = Regex::new(&type_def.pattern).unwrap();
        for value in column.iter() {
            if !reg.is_match(value) {
                return false;
            }
        }
        true
    });
    return type_list;
}

pub fn check_if_type_matches(value: &str, type_def: &Type) -> bool {
    let reg = Regex::new(&type_def.pattern).unwrap();
    reg.is_match(value)
}

#[derive(PartialEq, Debug)]
pub struct TypeList {
    map: TypesMap,
    list: TypeVec
}

impl TypeList {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            list: Vec::new()
        }
    }

    pub fn from(list: TypeVec) -> Self {
        let mut instance = Self::new();
        for item in list {
            instance.add_type(item);
        }
        return instance;
    }

    pub fn add_type(&mut self, type_config: Type) {
        if let None = self.map.get(&type_config.name) {
            self.list.push(type_config.clone());
        } else {
            self.list.retain(|t| t.name != type_config.name);
            self.list.push(type_config.clone());
        }
        self.map.insert(type_config.name.clone(), type_config.clone());
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
    fn get_all_values() {
        let mut types_list:TypesMap = HashMap::new();
        let string = Type {name: "string".to_owned(), pattern: "\\w".to_owned()};
        types_list.insert("string".to_owned(), string.clone());
        let t2 = Type {name: "t2".to_owned(), pattern: "w".to_owned()};
        types_list.insert("t2".to_owned(), t2.clone());
        let mut tl = TypeList::new();
        tl.add_type(string);
        tl.add_type(t2);
        assert_eq!(&types_list, tl.get_types_map())
    }

    #[test]
    fn type_map_to_vec() {
        let mut tl = TypeList::new();
        tl.add_type(Type::new("t", "t"));
        tl.add_type(Type::new("a", "a"));
        tl.add_type(Type::new("d", "d"));
        let expected = vec!(Type::new("t", "t"), Type::new("a", "a"), Type::new("d", "d"));
        assert_eq!(&expected, tl.get_types_vec());
    }

    #[test]
    fn type_map_to_vec_replace_existing() {
        let mut tl = TypeList::new();
        tl.add_type(Type::new("c", "t"));
        tl.add_type(Type::new("t", "t"));
        tl.add_type(Type::new("b", "t"));
        tl.add_type(Type::new("t", "a"));
        let expected = vec!(Type::new("c", "t"), Type::new("b", "t"), Type::new("t", "a"));
        assert_eq!(&expected, tl.get_types_vec());
    }

    #[test]
    fn all_strings() {
        let str1 = "str1".to_owned();
        let str2 = "str2".to_owned();
        let str3 = "str3".to_owned();
        let col = vec!(str1, str2, str3);
        let mut tl = TypeList::new();
        tl.add_type(Type::new("string", ".*"));
        let expected = vec!(Type::new("string", ".*"));
        assert_eq!(expected, get_matching_types(&col, tl.get_types_vec()));
    }

    #[test]
    fn match_multiple_types() {
        let str1 = "1".to_owned();
        let str2 = "2".to_owned();
        let str3 = "343".to_owned();
        let col = vec!(str1, str2, str3);
        let mut tl = TypeList::new();
        tl.add_type(Type::new("string", ".*"));
        tl.add_type(Type::new("int", r"^\d*$"));
        let expected = vec!(Type::new("string", ".*"), Type::new("int", r"^\d*$"));
        assert_eq!(expected, get_matching_types(&col, &tl.get_types_vec()));
    }

    #[test]
    fn match_multiple_types_one_not_matching() {
        let str1 = "1".to_owned();
        let str2 = "w2".to_owned();
        let str3 = "343".to_owned();
        let col = vec!(str1, str2, str3);
        let mut tl = TypeList::new();
        tl.add_type(Type::new("string", ".*"));
        tl.add_type(Type::new("int", r"^\d*$"));
        let expected = vec!(Type::new("string", ".*"));
        assert_eq!(expected, get_matching_types(&col, &tl.get_types_vec()));
    }
}
