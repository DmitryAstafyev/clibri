use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E>>(
    request: &protocol::StructD,
    scope: &mut Scope<'_, E, C>,
) -> Result<protocol::StructA, protocol::StructC> {
    let index = scope
        .context
        .get_index(scope.identification.uuid(), Alias::StructD);
    if index == 1 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructA);
        Ok(samples::struct_a::get())
    } else {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructC);
        Err(samples::struct_c::get())
    }
}
