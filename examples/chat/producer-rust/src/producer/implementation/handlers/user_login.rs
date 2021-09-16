use super::{
    broadcast, identification, pack, producer::Control, protocol, responses, Context, HandlerError,
};
use uuid::Uuid;

pub async fn process(
    context: &mut Context,
    filter: identification::Filter,
    uuid: Uuid,
    request: &protocol::UserLogin::Request,
    sequence: u32,
    control: &Control,
) -> Result<(), HandlerError> {
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let buffer = match responses::user_login::response(context, request, filter, control).await {
        Ok(conclusion) => match conclusion {
            responses::user_login::Response::Accepted((
                mut response,
                mut broadcast_userlogin,
                mut broadcast_message,
            )) => {
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
                pack(&sequence, &uuid, &mut response)?
            }
            responses::user_login::Response::Deny(mut response) => {
                pack(&sequence, &uuid, &mut response)?
            }
        },
        Err(mut error) => pack(&sequence, &uuid, &mut error)?,
    };
    control
        .send(buffer, Some(uuid))
        .map_err(|e| HandlerError::Processing(e.to_string()))?;
    for msg in broadcasting.iter_mut() {
        broadcast(msg, control)?;
    }
    Ok(())
}
