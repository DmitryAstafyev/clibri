use super::protocol;

#[derive(Debug, Clone)]
pub struct Options {
    pub reconnection: ReconnectionStrategy,
    pub request_timeout: u64,
    pub key: protocol::StructA,
}
impl Options {
    pub fn defualt(key: protocol::StructA) -> Self {
        Options {
            reconnection: ReconnectionStrategy::Reconnect(2000),
            request_timeout: 20000,
            key,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ReconnectionStrategy {
    Reconnect(u64),
    DoNotReconnect,
}
