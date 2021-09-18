use super::{broadcast, events, identification, pack, producer::Control, Context, EmitterError};
use uuid::Uuid;

pub async fn emit<E: std::error::Error>(
    uuid: Uuid,
    context: &mut Context,
    filter: identification::Filter,
    control: &Control,
) -> Result<(), EmitterError> {
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let (mut broadcat_userdisconnected, mut broadcast_message) =
        events::disconnected::emit::<E>(uuid, context, filter, control)
            .await
            .map_err(EmitterError::Emitting)?;
    broadcasting.push((
        broadcat_userdisconnected.0,
        pack(&0, &uuid, &mut broadcat_userdisconnected.1)?,
    ));
    if let Some(mut broadcast_message) = broadcast_message.take() {
        broadcasting.push((
            broadcast_message.0,
            pack(&0, &uuid, &mut broadcast_message.1)?,
        ));
    }
    for msg in broadcasting.iter_mut() {
        broadcast::<E>(msg, control)?;
    }
    Ok(())
}
