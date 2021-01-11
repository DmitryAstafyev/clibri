use super::context::{ ConnectionContext };
use std::sync::{ Arc, RwLock };
use uuid::Uuid;

pub enum ServerEvents<T> where T: ConnectionContext + Send + Sync {
    Connected(Uuid, Arc<RwLock<T>>),
    Disconnected(Uuid, Arc<RwLock<T>>),
    Received(Uuid, Arc<RwLock<T>>, Vec<u8>),
    Error(Option<Uuid>, String),
}
