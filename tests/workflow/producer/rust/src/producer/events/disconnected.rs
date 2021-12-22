use super::{identification, producer::Control, protocol, Context};
use crate::stat::Alias;
use clibri::server;
use uuid::Uuid;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: &identification::Filter<'_>,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    context.inc_stat(identification.uuid(), Alias::Disconnected);
    Ok(())
}
