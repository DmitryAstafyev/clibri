use super::{identification, producer::Control, protocol, scope::AnonymousScope, Context};
use clibri::server;
use std::str::FromStr;
use uuid::Uuid;

type BroadcastEventsMessage = (Vec<Uuid>, protocol::Events::Message);
type BroadcastEventsUserDisconnected = (Vec<Uuid>, protocol::Events::UserDisconnected);

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    event: protocol::ServerEvents::UserKickOff,
    scope: &mut AnonymousScope<'_, E, C>,
) -> Result<(BroadcastEventsMessage, BroadcastEventsUserDisconnected), String> {
    let uuid = Uuid::from_str(&event.uuid).map_err(|e| e.to_string())?;
    if let Some(user) = scope.context.remove_user(uuid).await {
        let msg = scope
            .context
            .add_message(&user.name, format!("User {} has been left", user.name))?;
        Ok((
            (scope.filter.except(&uuid), msg.clone()),
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
