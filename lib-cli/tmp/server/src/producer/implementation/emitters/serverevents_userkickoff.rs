use super::{
    broadcast, events, identification, producer::Control, protocol, unbound_pack, Context,
    EmitterError,
};
use fiber::server;
use uuid::Uuid;

pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::ServerEvents::UserKickOff,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let (mut broadcast_events_message, mut broadcast_events_userdisconnected) =
        events::serverevents_userkickoff::emit::<E, C>(event, filter, context, control)
            .await
            .map_err(EmitterError::Emitting)?;
    broadcasting.push((
        broadcast_events_message.0,
        unbound_pack(&0, &mut broadcast_events_message.1)?,
    ));
    broadcasting.push((
        broadcast_events_userdisconnected.0,
        unbound_pack(&0, &mut broadcast_events_userdisconnected.1)?,
    ));
    for msg in broadcasting.iter_mut() {
        broadcast::<E, C>(msg, control).await?;
    }
    Ok(())
}