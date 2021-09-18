use super::{identification, producer::Control, protocol, Context};
use uuid::Uuid;

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error>(
    uuid: Uuid,
    context: &mut Context,
    filter: identification::Filter,
    control: &Control,
) -> Result<(), String> {
    Ok(())
    // Err(String::from(
    //     "Event emitter \"connected\" isn't implemented",
    // ))
}
