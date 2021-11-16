use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;

type BroadcastEventsUserConnected = (Vec<Uuid>, protocol::Events::UserConnected);
type BroadcastEventsMessage = (Vec<Uuid>, protocol::Events::Message);

pub enum Response {
    Accept(
        (
            protocol::UserLogin::Accepted,
            BroadcastEventsUserConnected,
            BroadcastEventsMessage,
        ),
    ),
    Deny(protocol::UserLogin::Denied),
}

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::UserLogin::Request,
    control: &Control<E, C>,
) -> Result<Response, protocol::UserLogin::Err> {
    let uuid = identification.uuid();
    if context.is_user_exist(&request.username).await {
        Ok(Response::Deny(protocol::UserLogin::Denied {
            reason: String::from("User has been login already"),
        }))
    } else {
        context.add_user(uuid, &request.username).await;
        let msg = context
            .add_message(
                &request.username,
                format!("User {} has been join to chat", request.username),
            )
            .map_err(|e| protocol::UserLogin::Err { error: e })?;
        Ok(Response::Accept((
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
            (filter.except(uuid), msg),
        )))
    }
}
