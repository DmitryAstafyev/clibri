use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;
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

#[async_trait]
pub trait Control<E: std::error::Error>: Send + Clone {
    async fn shutdown(&self) -> Result<(), E>;
    async fn send(&self, buffer: Vec<u8>, client: Option<Uuid>) -> Result<(), E>;
    async fn disconnect(&self, client: Uuid) -> Result<(), E>;
    async fn disconnect_all(&self) -> Result<(), E>;
}

#[async_trait]
pub trait Impl<E: std::error::Error + Clone, C: Control<E> + Send + Clone>: Send {
    async fn listen(&mut self) -> Result<(), E>;
    fn observer(&mut self) -> Result<UnboundedReceiver<Events<E>>, E>;
    fn control(&self) -> C;
}
