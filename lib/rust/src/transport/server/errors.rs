#[derive(Debug, Clone)]
pub enum Errors {
    Create(String),
    AddStream(String),
    AcceptStream(String),
    CreateWS(String),
    NonBinaryData,
    FailSendBack(String),
    CannotClose(String),
    InvalidMessage(String),
    Other(String),
}
