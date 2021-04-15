#[derive(Debug, Clone)]
pub enum Errors {
    Create(String),
    AcceptStream(String),
    Other(String),
}
