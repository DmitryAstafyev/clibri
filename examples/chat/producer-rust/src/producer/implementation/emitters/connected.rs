use super::{broadcast, events, identification, pack, producer::Control, Context, EmitterError};
use uuid::Uuid;

pub async fn emit(
    uuid: Uuid,
    context: &mut Context,
    filter: identification::Filter,
    control: &Control,
) -> Result<(), EmitterError> {
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let (mut broadcast_userlogin, mut broadcast_message) =
        events::connected::emit(uuid, context, filter, control)
            .await
            .map_err(EmitterError::Emitting)?;
    broadcasting.push((
        broadcast_userlogin.0,
        pack(&0, &uuid, &mut broadcast_userlogin.1)?,
    ));
    if let Some(mut broadcast_message) = broadcast_message.take() {
        broadcasting.push((
            broadcast_message.0,
            pack(&0, &uuid, &mut broadcast_message.1)?,
        ));
    }
    for msg in broadcasting.iter_mut() {
        broadcast(msg, control)?;
    }
    Ok(())
}
