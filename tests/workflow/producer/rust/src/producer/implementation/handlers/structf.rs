
use super::{
    identification, pack, producer::Control, protocol, responses, Context, HandlerError,
    ProducerError,
};
use clibri::server;

pub async fn process<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::StructF,
    sequence: u32,
    control: &Control<E, C>,
) -> Result<(), HandlerError> {
    let uuid = identification.uuid();
    let buffer = match responses::structf::response(
        identification,
        filter,
        context,
        request,
        control,
    )
    .await
    {
        Ok(mut response) => pack(&sequence, &uuid, &mut response)?,
        Err(mut error) => pack(&sequence, &uuid, &mut error)?,
    };
    control
        .send(buffer, Some(uuid))
        .await
        .map_err(|e: ProducerError<E>| HandlerError::Processing(e.to_string()))
}
