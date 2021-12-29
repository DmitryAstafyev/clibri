
use super::{
    broadcast, identification, pack, producer::Control, protocol, responses, Context, HandlerError,
    ProducerError, scope::Scope,
};
use clibri::server;
use uuid::Uuid;

pub async fn process<E: server::Error, C: server::Control<E>>(
    identification: &identification::Identification,
    filter: &identification::Filter<'_>,
    context: &mut Context,
    request: &protocol::Message::Request,
    sequence: u32,
    control: &Control<E, C>,
) -> Result<(), HandlerError> {
    let mut scope: Scope<'_, E, C> = Scope::new(context, control, identification, filter);
    let uuid = identification.uuid();
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let buffer =
        match responses::message_request::response(request, &mut scope).await {
            Ok(conclusion) => match conclusion {
                responses::message_request::Response::Accept((
                    mut response,
                	mut broadcast_events_message,
                )) => {
                    broadcasting.push((
                        broadcast_events_message.0,
                        pack(&0, &uuid, &mut broadcast_events_message.1)?,
                    ));
                    pack(&sequence, &uuid, &mut response)?
                },
                responses::message_request::Response::Deny(mut response) => {
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
    scope.call().await;
    Ok(())
}    
