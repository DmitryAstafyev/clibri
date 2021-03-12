use std::collections::HashMap;
use std::cmp::{ PartialEq, Eq };

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EFilterMatchCondition {
    All,
    AtLeastOne,
    Equal,
}

#[derive(Debug, Clone)]
pub struct Identification {
    fp: HashMap<String, String>,
}

impl Identification {

    pub fn new() -> Self {
        Identification { fp: HashMap::new() }
    }

    pub fn set(&mut self, fp: HashMap<String, String>) {
        for (key, value) in fp {
            self.fp.remove(&key);
            self.fp.insert(key, value);
        }
    }

    pub fn remove(&mut self, key: String) {
        self.fp.remove(&key);
    }

    pub fn filter(&self, request: HashMap<String, String>, condition: EFilterMatchCondition) -> bool {
        fn is_match(key: String, value: String, request: HashMap<String, String>) -> bool {
            if let Some(v) = request.get(&key) {
                v == &*value
            } else {
                false
            }
        }
        if condition == EFilterMatchCondition::Equal && self.fp.len() != request.len() {
            return false;
        }
        for (key, value) in request.into_iter() {
            let matching: bool = is_match(key, value, self.fp.clone());
            if condition == EFilterMatchCondition::AtLeastOne && matching{
                return true;
            } else if !matching{
                return false;
            }
        }
        true
    }

}