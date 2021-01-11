use std::collections::HashMap;

pub trait Encodable {
    fn abduct(&mut self) -> Result<Vec<u8>, String>;
}

pub trait Context {

    fn send(&mut self, buffer: Vec<u8>) -> Result<(), String>;

    fn send_to(&mut self, ident: HashMap<String, String>, buffer: Vec<u8>) -> Result<(), String>;

}
