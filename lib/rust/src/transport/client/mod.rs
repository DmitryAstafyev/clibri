use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedReceiver;

pub trait Error: 'static + std::error::Error + Clone + Sync + Send {}

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
pub enum Event<E: Error> {
    Connected(SocketAddr),
    Disconnected,
    Error(E),
    Message(Message),
}

#[async_trait]
pub trait Control<E: Error>: Send + Clone {
    async fn shutdown(&self) -> Result<(), E>;
    async fn send(&self, msg: Message) -> Result<(), E>;
}

#[async_trait]
pub trait Impl<E: Error, C: Control<E> + Send + Clone>: Send {
    async fn connect(&mut self) -> Result<(), E>;
    fn observer(&mut self) -> Result<UnboundedReceiver<Event<E>>, E>;
    fn control(&self) -> C;
}
