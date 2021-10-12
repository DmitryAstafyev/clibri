use super::{identification, producer::Control, protocol, Context};
use fiber::server;
use uuid::Uuid;

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    Err(String::from(
        "Event emitter \"disconnected\" isn't implemented",
    ))
}
