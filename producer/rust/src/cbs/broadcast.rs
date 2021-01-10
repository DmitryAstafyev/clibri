use super::context::{ Context, Encodable };

pub trait Broadcasting<Identification>: Encodable {

    fn broadcast(&mut self, ident: Identification) -> Result<(), String>;

}
