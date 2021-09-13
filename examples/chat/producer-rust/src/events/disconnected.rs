use crate::{identification, producer::Control, protocol, Context};
use uuid::Uuid;

type BroadcastMessage = (Vec<Uuid>, protocol::Events::Message);

#[allow(unused_variables)]
pub async fn emit(
    uuid: Uuid,
    context: &mut Context,
    filter: identification::Filter,
    control: &Control,
) -> Result<BroadcastMessage, String> {
    Err(String::from(
        "Event emitter \"disconnected\" isn't implemented",
    ))
}
