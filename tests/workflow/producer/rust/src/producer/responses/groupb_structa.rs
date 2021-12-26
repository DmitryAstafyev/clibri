use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;
use uuid::Uuid;

pub enum Response {
    GroupBStructA(protocol::GroupB::StructA),
    GroupBGroupCStructA(protocol::GroupB::GroupC::StructA),
}

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E>>(
    request: &protocol::GroupB::StructA,
    scope: &mut Scope<'_, E, C>,
) -> Result<Response, protocol::GroupB::GroupC::StructB> {
    let index = scope
        .context
        .get_index(scope.identification.uuid(), Alias::GroupBStructA);
    if index == 1 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::GroupBStructA);
        Ok(Response::GroupBStructA(samples::group_b::struct_a::get()))
    } else if index == 2 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::GroupBGroupCStructA);
        Ok(Response::GroupBGroupCStructA(
            samples::group_b::group_c::struct_a::get(),
        ))
    } else {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::GroupBGroupCStructB);
        Err(samples::group_b::group_c::struct_b::get())
    }
}
