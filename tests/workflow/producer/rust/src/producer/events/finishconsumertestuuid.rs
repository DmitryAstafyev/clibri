use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;

type BroadcastFinishConsumerTestUuidBroadcast = (Vec<Uuid>, protocol::FinishConsumerTestUuidBroadcast);

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::FinishConsumerTestUuid,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<BroadcastFinishConsumerTestUuidBroadcast, String> {
    panic!("Handler for protocol::FinishConsumerTestUuid isn't implemented");
}