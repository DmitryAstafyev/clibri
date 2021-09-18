use super::{
    broadcast, identification, pack, producer::Control, protocol, responses, Context, HandlerError,
    ProducerError,
};
use uuid::Uuid;

pub async fn process<E: std::error::Error>(
    context: &mut Context,
    filter: identification::Filter,
    uuid: Uuid,
    request: &protocol::Message::Request,
    sequence: u32,
    control: &Control,
) -> Result<(), HandlerError> {
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let buffer = match responses::message::response(uuid, context, request, filter, control).await {
        Ok(conclusion) => match conclusion {
            responses::message::Response::Accepted((mut response, mut broadcast_message)) => {
                broadcasting.push((
                    broadcast_message.0,
                    pack(&0, &uuid, &mut broadcast_message.1)?,
                ));
                pack(&sequence, &uuid, &mut response)?
            }
            responses::message::Response::Deny(mut response) => {
                pack(&sequence, &uuid, &mut response)?
            }
        },
        Err(mut error) => pack(&sequence, &uuid, &mut error)?,
    };
    control
        .send(buffer, Some(uuid))
        .map_err(|e: ProducerError<E>| HandlerError::Processing(e.to_string()))?;
    for msg in broadcasting.iter_mut() {
        broadcast::<E>(msg, control)?;
    }
    Ok(())
}
