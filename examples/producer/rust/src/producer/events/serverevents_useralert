use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;

type BroadcastEventsMessage = (Vec<Uuid>, protocol::Events::Message);
type BroadcastEventsUserConnected = Option<(Vec<Uuid>, protocol::Events::UserConnected)>;

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::ServerEvents::UserAlert,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(BroadcastEventsMessage, BroadcastEventsUserConnected), String> {
    panic!("Handler for protocol::ServerEvents::UserAlert isn't implemented");
}