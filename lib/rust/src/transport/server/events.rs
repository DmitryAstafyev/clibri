use uuid::Uuid;
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
