use super::{identification, producer::Control, protocol, Context};
use uuid::Uuid;

type BroadcastUserConnected = (Vec<Uuid>, protocol::Events::UserConnected);
type BroadcastMessage = Option<(Vec<Uuid>, protocol::Events::Message)>;

pub enum Response {
    Accepted(
        (
            protocol::UserLogin::Accepted,
            BroadcastUserConnected,
            BroadcastMessage,
        ),
    ),
    Deny(protocol::UserLogin::Denied),
}

#[allow(unused_variables)]
pub async fn response(
    context: &mut Context,
    request: &protocol::UserLogin::Request,
    filter: identification::Filter,
    control: &Control,
) -> Result<Response, protocol::UserLogin::Err> {
    if context.is_user_exist(&request.username).await {
        Ok(Response::Deny(protocol::UserLogin::Denied {
            reason: String::from("User has been login already"),
        }))
    } else {
        let uuid = context.add_user(&request.username).await;
        Ok(Response::Accepted((
            protocol::UserLogin::Accepted {
                uuid: uuid.to_string(),
            },
            (
                filter.except(uuid),
                protocol::Events::UserConnected {
                    username: request.username.clone(),
                    uuid: uuid.to_string(),
                },
            ),
            Some((
                filter.except(uuid),
                protocol::Events::Message {
                    timestamp: 0,
                    user: request.username.clone(),
                    message: String::new(),
                    uuid: uuid.to_string(),
                },
            )),
        )))
    }
}
