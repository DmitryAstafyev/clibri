use super::{ Messages, Protocol, Identification };
use super::buffer::{ Buffer };
use super::context::{ Context };
use fiber::server::context::{ ConnectionContext };
use uuid::Uuid;
use std::sync::{ Arc, RwLock, Mutex };

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
    cx: Arc<Mutex<dyn ConnectionContext>>
}

impl Consumer {

    pub fn new(cx: dyn ConnectionContext) -> Self {
        let uuid: Uuid = Uuid::new_v4();
        Consumer {
            uuid,
            buffer: Buffer::new(uuid),
            cx: Arc::new(Mutex::new(cx)),
        }
    }

    pub fn read(&mut self, buffer: Vec<u8>) -> Result<Messages, String> {
        Err("".to_owned())
    }

    pub fn get_cx(&mut self) -> impl Context<Identification> {
        Cx {}
    } 


}