use super::{identification, producer::Control, protocol, Context};
use fiber::server;
use std::str::FromStr;

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    beacon: protocol::Events::UserDisconnected,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    panic!("Handler for protocol::Events::UserDisconnected isn't implemented");
}