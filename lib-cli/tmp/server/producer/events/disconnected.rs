use super::{identification, producer::Control, protocol, Context};
use fiber::server;
use uuid::Uuid;

type BroadcastEventsMessage = Option<(Vec<Uuid>, protocol::Events::Message)>;
type BroadcastEventsUserDisconnected = (Vec<Uuid>, protocol::Events::UserDisconnected);

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(BroadcastEventsMessage, BroadcastEventsUserDisconnected), String> {
    panic!("Handler for protocol::disconnected isn't implemented");
}