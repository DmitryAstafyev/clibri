use super::{identification, producer::Control, protocol, scope::Scope, Context};
use clibri::server;
use uuid::Uuid;

type BroadcastEventsMessage = Option<(Vec<Uuid>, protocol::Events::Message)>;
type BroadcastEventsUserDisconnected = (Vec<Uuid>, protocol::Events::UserDisconnected);

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    scope: &mut Scope<'_, E, C>,
) -> Result<(BroadcastEventsMessage, BroadcastEventsUserDisconnected), String> {
    let uuid = scope.identification.uuid();
    if let Some(user) = scope.context.remove_user(uuid).await {
        let msg = scope
            .context
            .add_message(&user.name, format!("User {} has been left", user.name))?;
        Ok((
            Some((scope.filter.except(&uuid), msg.clone())),
            (
                scope.filter.except(&uuid),
                protocol::Events::UserDisconnected {
                    username: msg.user,
                    uuid: uuid.to_string(),
                },
            ),
        ))
    } else {
        Err(format!("User {} doesn't exist", uuid))
    }
}
