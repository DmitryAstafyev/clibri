use super::{identification, producer::Control, protocol, Context};
use uuid::Uuid;

type BroadcastMessage = (Vec<Uuid>, protocol::Events::Message);

pub enum Response {
    Accepted((protocol::Message::Accepted, BroadcastMessage)),
    Deny(protocol::Message::Denied),
}

#[allow(unused_variables)]
pub async fn response(
    context: &mut Context,
    request: &protocol::Message::Request,
    filter: identification::Filter,
    control: &Control,
) -> Result<Response, protocol::Message::Err> {
    if context.is_user_exist(&request.user).await {
        Ok(Response::Deny(protocol::Message::Denied {
            reason: String::from("User doesn't exist"),
        }))
    } else {
        let uuid = Uuid::new_v4();
        //TODO: add message
        Ok(Response::Accepted((
            protocol::Message::Accepted {
                uuid: uuid.to_string(),
            },
            (
                filter.except(uuid),
                protocol::Events::Message {
                    timestamp: 0,
                    user: request.user.clone(),
                    message: request.message.clone(),
                    uuid: uuid.to_string(),
                },
            ),
        )))
    }
}
