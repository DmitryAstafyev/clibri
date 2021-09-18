use async_trait::async_trait;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

pub type Sending = (Vec<u8>, Option<Uuid>);

pub enum Events<E: std::error::Error> {
    Ready,
    Shutdown,
    Connected(Uuid),
    Disconnected(Uuid),
    Received(Uuid, Vec<u8>),
    Error(Option<Uuid>, String),
    ConnectionError(Option<Uuid>, E),
    ServerError(E),
}

pub enum Control {
    Shutdown,
    Disconnect(Uuid),
}

#[async_trait]
pub trait Impl<E: std::error::Error>: Send {
    async fn listen(&mut self) -> Result<(), E>;
    fn observer(&mut self) -> Result<UnboundedReceiver<Events<E>>, E>;
    fn sender(&self) -> UnboundedSender<Sending>;
    fn control(&self) -> UnboundedSender<Control>;
}
