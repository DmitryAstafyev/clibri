use super::{identification, producer::Control, protocol, Context};
use fiber::server;
use uuid::Uuid;

type BroadcastEventsMessage = Option<(Vec<Uuid>, protocol::Events::Message)>;
type BroadcastEventsUserDisconnected = (Vec<Uuid>, protocol::Events::UserDisconnected);

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(BroadcastEventsMessage, BroadcastEventsUserDisconnected), String> {
    let uuid = identification.uuid();
    if let Some(user) = context.remove_user(uuid).await {
        let msg = context.add_message(&user.name, format!("User {} has been left", user.name))?;
        Ok((
            Some((filter.except(uuid), msg.clone())),
            (
                filter.except(uuid),
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
