use super::{identification, producer::Control, protocol, Context};
use uuid::Uuid;

type BroadcastUserDisconnected = (Vec<Uuid>, protocol::Events::UserDisconnected);
type BroadcastMessage = Option<(Vec<Uuid>, protocol::Events::Message)>;

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control,
) -> Result<(BroadcastUserDisconnected, BroadcastMessage), String> {
    let uuid = identification.uuid();
    if let Some(user) = context.remove_user(uuid).await {
        let msg = context.add_message(&user.name, format!("User {} has been left", user.name))?;
        Ok((
            (
                filter.except(uuid),
                protocol::Events::UserDisconnected {
                    username: msg.user.clone(),
                    uuid: uuid.to_string(),
                },
            ),
            Some((filter.except(uuid), msg)),
        ))
    } else {
        Err(format!("User {} doesn't exist", uuid))
    }
}
