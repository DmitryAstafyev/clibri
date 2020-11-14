use uuid::Uuid;
use tungstenite::protocol::CloseFrame;

pub enum Error {
    Parsing(String),
    ReadSocket(String),
    Channel(String),
}

pub enum Messages<T: Send + Sync + Clone + 'static> {
    Error { uuid: Uuid, error: Error },
    Disconnect { uuid: Uuid, frame: Option<CloseFrame<'static>> },
    Message { uuid: Uuid, msg: T },
    Text { uuid: Uuid, text: String }
}