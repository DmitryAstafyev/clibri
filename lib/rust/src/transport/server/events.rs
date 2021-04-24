use uuid::Uuid;
use super::{
    errors::{
        Errors
    }
};
pub enum Events {
    Ready,
    Connected(Uuid),
    Disconnected(Uuid),
    Received(Uuid, Vec<u8>),
    Error(Option<Uuid>, String),
    ConnectionError(Option<Uuid>, Errors),
    ServerError(Errors),
}
