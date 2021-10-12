use super::{identification, producer::Control, Context};
use fiber::server;

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    Err(String::from(
        "Event emitter \"connected\" isn't implemented",
    ))
}
