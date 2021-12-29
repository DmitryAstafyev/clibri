use super::{
    broadcast, events, identification, producer::Control, protocol, unbound_pack, Context,
    EmitterError, scope::AnonymousScope,
};
use clibri::server;
use uuid::Uuid;

pub async fn emit<E: server::Error, C: server::Control<E>>(
    event: protocol::EventB,
    filter: &identification::Filter<'_>,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    let mut scope: AnonymousScope<'_, E, C> = AnonymousScope::new(context, control, filter);
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let mut broadcast_structc =
        events::eventb::emit(event, &mut scope)
            .await
            .map_err(EmitterError::Emitting)?;
    broadcasting.push((
        broadcast_structc.0,
        unbound_pack(&0, &mut broadcast_structc.1)?,
    ));
    for msg in broadcasting.iter_mut() {
        broadcast::<E, C>(msg, control).await?;
    }
    scope.call().await;
    Ok(())
}