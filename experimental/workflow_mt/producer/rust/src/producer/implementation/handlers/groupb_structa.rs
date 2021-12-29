use super::{
    broadcast, hub, identification, pack, producer::Control, protocol, responses, Context,
    HandlerError, ProducerError,
};
use clibri::server;
use uuid::Uuid;

pub async fn process<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: hub::filter::Filter,
    context: &Context,
    request: &protocol::GroupB::StructA,
    sequence: u32,
    control: &Control<E, C>,
) -> Result<(), HandlerError> {
    let uuid = identification.uuid();
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let buffer = match responses::groupb_structa::response(
        identification,
        filter,
        context,
        request,
        control,
    )
    .await
    {
        Ok(conclusion) => match conclusion {
            responses::groupb_structa::Response::GroupBStructA(mut response) => {
                pack(&sequence, &uuid, &mut response)?
            }
            responses::groupb_structa::Response::GroupBGroupCStructA(mut response) => {
                pack(&sequence, &uuid, &mut response)?
            }
        },
        Err(mut error) => pack(&sequence, &uuid, &mut error)?,
    };
    control
        .send(buffer, Some(uuid))
        .await
        .map_err(|e: ProducerError<E>| HandlerError::Processing(e.to_string()))?;
    for msg in broadcasting.iter_mut() {
        broadcast::<E, C>(msg, control).await?;
    }
    Ok(())
}