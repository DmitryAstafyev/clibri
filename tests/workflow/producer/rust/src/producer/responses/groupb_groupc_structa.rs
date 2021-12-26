use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E>>(
    request: &protocol::GroupB::GroupC::StructA,
    scope: &mut Scope<'_, E, C>,
) -> Result<protocol::GroupB::GroupC::StructB, protocol::GroupA::StructB> {
    let index = scope
        .context
        .get_index(scope.identification.uuid(), Alias::GroupBGroupCStructA);
    if index == 1 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::GroupBGroupCStructB);
        Ok(samples::group_b::group_c::struct_b::get())
    } else {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::GroupAStructB);
        Err(samples::group_a::struct_b::get())
    }
}
