use super::{ Messages, Protocol, Identification };
use super::buffer::{ Buffer };
use super::context::{ Context };
use fiber::server::context::{ ConnectionContext };
use fiber_transport_server::connection_context::{ ConnectionContext as ServerConnectionContext };
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

pub struct Consumer<T> where T: ConnectionContext + Send + Sync {
    uuid: Uuid,
    buffer: Buffer<Protocol>,
    cx: Arc<Mutex<T>>
}

impl<T> Consumer<T> where T: ConnectionContext + Send + Sync  {

    pub fn new(cx: Arc<Mutex<T>>) -> Self {
        let uuid: Uuid = Uuid::new_v4();
        Consumer {
            uuid,
            buffer: Buffer::new(uuid),
            cx,
        }
    }

    pub fn read(&mut self, buffer: Vec<u8>) -> Result<Messages, String> {
        Err("".to_owned())
    }

    pub fn get_cx(&mut self) -> impl Context<Identification> {
        Cx {}
    } 


}