use super::{identification, producer::Control, protocol, Context};
use uuid::Uuid;

type BroadcastUserConnected = (Vec<Uuid>, protocol::Events::UserConnected);
type BroadcastMessage = Option<(Vec<Uuid>, protocol::Events::Message)>;

#[allow(unused_variables)]
pub async fn emit(
    uuid: Uuid,
    context: &mut Context,
    filter: identification::Filter,
    control: &Control,
) -> Result<(BroadcastUserConnected, BroadcastMessage), String> {
    Err(String::from(
        "Event emitter \"connected\" isn't implemented",
    ))
}
