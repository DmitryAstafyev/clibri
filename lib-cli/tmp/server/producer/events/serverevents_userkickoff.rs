use super::{identification, producer::Control, protocol, Context};
use fiber::server;
use std::str::FromStr;

type BroadcastEventsMessage = (Vec<Uuid>, protocol::Events::Message);
type BroadcastEventsUserConnected = (Vec<Uuid>, protocol::Events::UserConnected);

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::ServerEvents::UserKickOff,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(BroadcastEventsMessage, BroadcastEventsUserConnected), String> {
    panic!("Handler for protocol::ServerEvents::UserKickOff isn't implemented");
}