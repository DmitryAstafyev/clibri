use super::{identification, producer::Control, protocol, Context};
use uuid::Uuid;

type BroadcastMessage = (Vec<Uuid>, protocol::Events::Message);

pub enum Response {
    Accepted((protocol::Message::Accepted, BroadcastMessage)),
    Deny(protocol::Message::Denied),
}

#[allow(unused_variables)]
pub async fn response(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::Message::Request,
    control: &Control,
) -> Result<Response, protocol::Message::Err> {
    let uuid = identification.uuid();
    if !context.is_user_exist(&request.user).await {
        Ok(Response::Deny(protocol::Message::Denied {
            reason: String::from("User doesn't exist"),
        }))
    } else {
        match context.add_message(&request.user, request.message.clone()) {
            Ok(msg) => Ok(Response::Accepted((
                protocol::Message::Accepted {
                    uuid: msg.uuid.clone(),
                },
                (filter.except(uuid), msg),
            ))),
            Err(err) => Err(protocol::Message::Err { error: err }),
        }
    }
}
