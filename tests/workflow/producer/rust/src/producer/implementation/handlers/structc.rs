
use super::{
    broadcast, identification, pack, producer::Control, protocol, responses, Context, HandlerError,
    ProducerError,
};
use clibri::server;
use uuid::Uuid;

pub async fn process<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::StructC,
    sequence: u32,
    control: &Control<E, C>,
) -> Result<(), HandlerError> {
    let uuid = identification.uuid();
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let buffer =
        match responses::structc::response(identification, filter, context, request, control).await {
            Ok(conclusion) => match conclusion {
                responses::structc::Response::CaseB(mut response) => {
                    pack(&sequence, &uuid, &mut response)?
                },
                responses::structc::Response::CaseF(mut response) => {
                    pack(&sequence, &uuid, &mut response)?
                },
                responses::structc::Response::CaseD(mut response) => {
                    pack(&sequence, &uuid, &mut response)?
                },
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
