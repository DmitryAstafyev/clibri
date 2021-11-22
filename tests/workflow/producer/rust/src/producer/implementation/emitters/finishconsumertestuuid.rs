use super::{
    broadcast, events, identification, producer::Control, protocol, unbound_pack, Context,
    EmitterError,
};
use clibri::server;
use uuid::Uuid;

pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::FinishConsumerTestUuid,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let mut broadcast_finishconsumertestuuidbroadcast =
        events::finishconsumertestuuid::emit::<E, C>(event, filter, context, control)
            .await
            .map_err(EmitterError::Emitting)?;
    broadcasting.push((
        broadcast_finishconsumertestuuidbroadcast.0,
        unbound_pack(&0, &mut broadcast_finishconsumertestuuidbroadcast.1)?,
    ));
    for msg in broadcasting.iter_mut() {
        broadcast::<E, C>(msg, control).await?;
    }
    Ok(())
}