use super::{
    identification, pack, producer::Control, protocol, responses, Context, HandlerError,
    ProducerError,
};
use uuid::Uuid;

pub async fn process<E: std::error::Error>(
    context: &mut Context,
    filter: identification::Filter,
    uuid: Uuid,
    request: &protocol::Messages::Request,
    sequence: u32,
    control: &Control,
) -> Result<(), HandlerError> {
    let buffer = match responses::messages::response(uuid, context, request, filter, control).await
    {
        Ok(mut response) => pack(&sequence, &uuid, &mut response)?,
        Err(mut error) => pack(&sequence, &uuid, &mut error)?,
    };
    control
        .send(buffer, Some(uuid))
        .map_err(|e: ProducerError<E>| HandlerError::Processing(e.to_string()))
}