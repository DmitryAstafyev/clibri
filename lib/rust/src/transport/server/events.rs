use uuid::Uuid;

pub enum Events {
    Connected(Uuid),
    Disconnected(Uuid),
    Received(Uuid, Vec<u8>),
    Error(Option<Uuid>, String),
}
