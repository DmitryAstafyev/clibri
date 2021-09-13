use super::{broadcast, pack, EmitterError};
use crate::{events, identification, producer::Control, Context};
use uuid::Uuid;

pub async fn emit(
    uuid: Uuid,
    context: &mut Context,
    filter: identification::Filter,
    control: &Control,
) -> Result<(), EmitterError> {
    let mut broadcast_message = events::disconnected::emit(uuid, context, filter, control)
        .await
        .map_err(EmitterError::Emitting)?;
    broadcast(
        &mut (
            broadcast_message.0,
            pack(&0, &uuid, &mut broadcast_message.1)?,
        ),
        control,
    )?;
    Ok(())
}
