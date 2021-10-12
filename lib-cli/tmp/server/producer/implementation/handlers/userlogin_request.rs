
use super::{
    broadcast, identification, pack, producer::Control, protocol, responses, Context, HandlerError,
    ProducerError,
};
use fiber::server;
use uuid::Uuid;

pub async fn process<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::UserLogin::Request,
    sequence: u32,
    control: &Control<E, C>,
) -> Result<(), HandlerError> {
    let uuid = identification.uuid();
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let buffer =
        match responses::userlogin_request::response(identification, filter, context, request, control).await {
            Ok(conclusion) => match conclusion {
                responses::userlogin_request::Response::Accept((
                    mut response,
                	mut broadcast_events_userconnected,
                	mut broadcast_events_message,,
                )) => {
                    broadcasting.push((
                        broadcast_events_userconnected.0,
                        pack(&0, &uuid, &mut broadcast_events_userconnected.1)?,
                    ));
                    broadcasting.push((
                        broadcast_events_message.0,
                        pack(&0, &uuid, &mut broadcast_events_message.1)?,
                    ));
                    pack(&sequence, &uuid, &mut response)?
                },
                responses::userlogin_request::Response::Deny(mut response) => {
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
