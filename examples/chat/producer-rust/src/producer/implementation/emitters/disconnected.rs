use super::{broadcast, events, identification, pack, producer::Control, Context, EmitterError};
use uuid::Uuid;

pub async fn emit<E: std::error::Error>(
    uuid: Uuid,
    context: &mut Context,
    filter: identification::Filter,
    control: &Control,
) -> Result<(), EmitterError> {
    let mut broadcast_message = events::disconnected::emit::<E>(uuid, context, filter, control)
        .await
        .map_err(EmitterError::Emitting)?;
    broadcast::<E>(
        &mut (
            broadcast_message.0,
            pack(&0, &uuid, &mut broadcast_message.1)?,
        ),
        control,
    )?;
    Ok(())
}
