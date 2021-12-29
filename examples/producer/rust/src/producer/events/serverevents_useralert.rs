use super::{identification, producer::Control, protocol, Context, scope::AnonymousScope};
use clibri::server;
use uuid::Uuid;

type BroadcastEventsMessage = (Vec<Uuid>, protocol::Events::Message);
type BroadcastEventsUserConnected = Option<(Vec<Uuid>, protocol::Events::UserConnected)>;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    event: protocol::ServerEvents::UserAlert,
    scope: &mut AnonymousScope<'_, E, C>,
) -> Result<(BroadcastEventsMessage, BroadcastEventsUserConnected), String> {
    panic!("Handler for protocol::ServerEvents::UserAlert isn't implemented");
}