use super::errors::Error;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub enum Message {
    /// A text WebSocket message
    Text(String),
    /// A binary WebSocket message
    Binary(Vec<u8>),
    /// A ping message with the specified payload
    ///
    /// The payload here must have a length less than 125 bytes
    Ping(Vec<u8>),
    /// A pong message with the specified payload
    ///
    /// The payload here must have a length less than 125 bytes
    Pong(Vec<u8>),
}

#[derive(Clone, Debug)]
pub enum Event {
    Connected(SocketAddr),
    Disconnected,
    Error(Error),
    Message(Message),
}
