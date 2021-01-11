use super::context::{ Context, Encodable };

pub trait Broadcasting: Encodable {

    fn broadcast(&mut self, ident: HashMap<String, String>) -> Result<(), String>;

}
