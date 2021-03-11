use std::collections::HashMap;
use super::consumer_identification::{EFilterMatchCondition};

pub trait Context {

    fn send(&self, buffer: Vec<u8>) -> Result<(), String>;

    fn send_to(&self, buffer: Vec<u8>, filter: HashMap<String, String>, condition: EFilterMatchCondition) -> Result<(), String>;

}
