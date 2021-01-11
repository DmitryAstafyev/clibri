use super::context::{ ConnectionContext };
use std::sync::{ Arc, Mutex };
use uuid::Uuid;

pub enum ServerEvents<T> where T: ConnectionContext + Send + Sync {
    Connected(Uuid, Arc<Mutex<T>>),
    Disconnected(Uuid, Arc<Mutex<T>>),
    Received(Uuid, Arc<Mutex<T>>, Vec<u8>),
    Error(Option<Uuid>, String),
}
