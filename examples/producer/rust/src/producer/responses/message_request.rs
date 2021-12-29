use super::{identification, producer::Control, protocol, scope::Scope, Context};
use clibri::server;
use uuid::Uuid;

type BroadcastEventsMessage = (Vec<Uuid>, protocol::Events::Message);

pub enum Response {
    Accept((protocol::Message::Accepted, BroadcastEventsMessage)),
    Deny(protocol::Message::Denied),
}

#[allow(unused_variables)]
pub async fn response<E: server::Error, C: server::Control<E>>(
    request: &protocol::Message::Request,
    scope: &mut Scope<'_, E, C>,
) -> Result<Response, protocol::Message::Err> {
    let uuid = scope.identification.uuid();
    if !scope.context.is_user_exist(&request.user).await {
        Ok(Response::Deny(protocol::Message::Denied {
            reason: String::from("User doesn't exist"),
        }))
    } else {
        match scope
            .context
            .add_message(&request.user, request.message.clone())
        {
            Ok(msg) => Ok(Response::Accept((
                protocol::Message::Accepted {
                    uuid: msg.uuid.clone(),
                },
                (scope.filter.except(&uuid), msg),
            ))),
            Err(err) => Err(protocol::Message::Err { error: err }),
        }
    }
}
