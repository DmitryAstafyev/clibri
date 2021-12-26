use super::{identification, producer::Control, scope::Scope, Context};
use crate::stat::Alias;
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    scope: &mut Scope<'_, E, C>,
) -> Result<(), String> {
    scope.context.add_stat(scope.identification.uuid());
    scope
        .context
        .inc_stat(scope.identification.uuid(), Alias::Connected);
    Ok(())
}
