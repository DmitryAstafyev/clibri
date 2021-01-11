use super::buffer::Buffer;
use super::consumer_identification::Identification;
use super::context::Context;
use super::{Messages, Protocol};
use fiber::server::context::ConnectionContext;
use fiber_transport_server::connection_context::ConnectionContext as ServerConnectionContext;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use uuid::Uuid;

pub struct Cx<T>
where
    T: ConnectionContext + Send + Sync,
{
    cx: Arc<Mutex<T>>,
}

impl<T> Context for Cx<T>
where
    T: ConnectionContext + Send + Sync,
{
    fn send(&mut self, buffer: Vec<u8>) -> Result<(), String> {
        Ok(())
    }

    fn send_to(&mut self, ident: HashMap<String, String>, buffer: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
}

pub struct Consumer<T>
where
    T: ConnectionContext + Send + Sync,
{
    uuid: Uuid,
    buffer: Buffer<Protocol>,
    cx: Arc<Mutex<T>>,
    identification: Identification,
}

impl<T> Consumer<T>
where
    T: ConnectionContext + Send + Sync,
{
    pub fn new(cx: Arc<Mutex<T>>) -> Self {
        let uuid: Uuid = Uuid::new_v4();
        Consumer {
            uuid,
            buffer: Buffer::new(uuid),
            cx,
            identification: Identification::new(),
        }
    }

    pub fn read(&mut self, buffer: Vec<u8>) -> Result<Messages, String> {
        Err("".to_owned())
    }

    pub fn get_cx(&mut self) -> impl Context {
        Cx {
            cx: self.cx.clone(),
        }
    }
}
