use super::{ Messages, Protocol, Identification };
use super::buffer::{ Buffer };
use super::context::{ Context };
use uuid::Uuid;

pub struct Cx {

}

impl Context<Identification> for Cx {

    fn send(&mut self, buffer: Vec<u8>) -> Result<(), String> {
        Ok(())
    }

    fn send_to(&mut self, ident: Identification, buffer: Vec<u8>) -> Result<(), String> {
        Ok(())
    }

}

pub struct Consumer {
    uuid: Uuid,
    buffer: Buffer<Protocol>,
}

impl Consumer {

    pub fn new() -> Self {
        let uuid: Uuid = Uuid::new_v4();
        Consumer {
            uuid,
            buffer: Buffer::new(uuid),
        }
    }

    pub fn read(&mut self, buffer: Vec<u8>) -> Result<Messages, String> {
        Err("".to_owned())
    }

    pub fn get_cx(&mut self) -> impl Context<Identification> {
        Cx {}
    } 


}