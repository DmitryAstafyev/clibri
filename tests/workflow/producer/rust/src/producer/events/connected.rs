use super::{identification, producer::Control, Context};
use crate::stat::Alias;
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: &identification::Filter<'_>,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    context.add_stat(identification.uuid());
    context.inc_stat(identification.uuid(), Alias::Connected);
    Ok(())
}
