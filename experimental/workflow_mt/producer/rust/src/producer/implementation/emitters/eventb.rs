use super::{
    broadcast, events, hub, identification, producer::Control, protocol, unbound_pack, Context,
    EmitterError,
};
use clibri::server;
use uuid::Uuid;

pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::EventB,
    filter: hub::filter::Filter,
    context: &Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let mut broadcast_structc = events::eventb::emit::<E, C>(event, filter, context, control)
        .await
        .map_err(EmitterError::Emitting)?;
    broadcasting.push((
        broadcast_structc.0,
        unbound_pack(&0, &mut broadcast_structc.1)?,
    ));
    for msg in broadcasting.iter_mut() {
        broadcast::<E, C>(msg, control).await?;
    }
    Ok(())
}
