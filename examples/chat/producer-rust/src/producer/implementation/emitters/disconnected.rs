use super::{broadcast, events, identification, pack, producer::Control, Context, EmitterError};
use fiber::server;
use uuid::Uuid;

pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let (mut broadcat_userdisconnected, mut broadcast_message) =
        events::disconnected::emit::<E, C>(identification, filter, context, control)
            .await
            .map_err(EmitterError::Emitting)?;
    broadcasting.push((
        broadcat_userdisconnected.0,
        pack(&0, &identification.uuid(), &mut broadcat_userdisconnected.1)?,
    ));
    if let Some(mut broadcast_message) = broadcast_message.take() {
        broadcasting.push((
            broadcast_message.0,
            pack(&0, &identification.uuid(), &mut broadcast_message.1)?,
        ));
    }
    for msg in broadcasting.iter_mut() {
        broadcast::<E, C>(msg, control).await?;
    }
    Ok(())
}
