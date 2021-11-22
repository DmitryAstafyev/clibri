use super::protocol;

#[derive(Debug, Clone)]
pub struct Options {
    pub reconnection: ReconnectionStrategy,
    pub key: protocol::StructA,
}
impl Options {
    pub fn defualt(key: protocol::StructA) -> Self {
        Options {
            reconnection: ReconnectionStrategy::Reconnect(2000),
            key,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ReconnectionStrategy {
    Reconnect(u64),
    DoNotReconnect,
}