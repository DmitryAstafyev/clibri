use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;
use uuid::Uuid;

type BroadcastStructD = (Vec<Uuid>, protocol::StructD);

pub enum Response {
    RootA((protocol::StructA, BroadcastStructD)),
    RootB(protocol::StructB),
}

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E>>(
    request: &protocol::GroupA::StructA,
    scope: &mut Scope<'_, E, C>,
) -> Result<Response, protocol::GroupA::StructB> {
    let index = scope
        .context
        .get_index(scope.identification.uuid(), Alias::GroupAStructA);
    if index == 1 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructA);
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructD);
        Ok(Response::RootA((
            samples::struct_a::get(),
            (vec![scope.identification.uuid()], samples::struct_d::get()),
        )))
    } else if index == 2 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructB);
        Ok(Response::RootB(samples::struct_b::get()))
    } else {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::GroupAStructB);
        Err(samples::group_a::struct_b::get())
    }
}
