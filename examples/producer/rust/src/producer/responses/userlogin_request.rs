use super::{identification, producer::Control, protocol, scope::Scope, Context};
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
pub async fn response<E: server::Error, C: server::Control<E>>(
    request: &protocol::UserLogin::Request,
    scope: &mut Scope<'_, E, C>,
) -> Result<Response, protocol::UserLogin::Err> {
    let uuid = scope.identification.uuid();
    if scope.context.is_user_exist(&request.username).await {
        Ok(Response::Deny(protocol::UserLogin::Denied {
            reason: String::from("User has been login already"),
        }))
    } else {
        scope.context.add_user(uuid, &request.username).await;
        let msg = scope
            .context
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
                scope.filter.except(&uuid),
                protocol::Events::UserConnected {
                    username: request.username.clone(),
                    uuid: uuid.to_string(),
                },
            ),
            (scope.filter.except(&uuid), msg),
        )))
    }
}
