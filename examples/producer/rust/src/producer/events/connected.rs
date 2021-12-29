use super::{identification, producer::Control, scope::Scope, Context};
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    scope: &mut Scope<'_, E, C>,
) -> Result<(), String> {
    Ok(())
}
