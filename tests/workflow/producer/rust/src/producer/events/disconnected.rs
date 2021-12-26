use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::stat::Alias;
use clibri::server;
use uuid::Uuid;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    scope: &mut Scope<'_, E, C>,
) -> Result<(), String> {
    scope
        .context
        .inc_stat(scope.identification.uuid(), Alias::Disconnected);
    Ok(())
}
