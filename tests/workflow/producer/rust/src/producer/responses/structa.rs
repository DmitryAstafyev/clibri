use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;
use uuid::Uuid;

type BroadcastStructD = (Vec<Uuid>, protocol::StructD);
type BroadcastStructF = (Vec<Uuid>, protocol::StructF);
type BroadcastStructJ = (Vec<Uuid>, protocol::StructJ);

pub enum Response {
    CaseB((protocol::StructB, BroadcastStructD, BroadcastStructF)),
    CaseC(protocol::StructC),

    CaseD((protocol::StructD, BroadcastStructJ)),
}

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E>>(
    request: &protocol::StructA,
    scope: &mut Scope<'_, E, C>,
) -> Result<Response, protocol::StructE> {
    let index = scope
        .context
        .get_index(scope.identification.uuid(), Alias::StructA);
    if index == 1 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructB);
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructD);
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructF);
        Ok(Response::CaseB((
            samples::struct_b::get(),
            (vec![scope.identification.uuid()], samples::struct_d::get()),
            (vec![scope.identification.uuid()], samples::struct_f::get()),
        )))
    } else if index == 2 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructC);
        Ok(Response::CaseC(samples::struct_c::get()))
    } else if index == 3 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructD);
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructJ);
        Ok(Response::CaseD((
            samples::struct_d::get(),
            (vec![scope.identification.uuid()], samples::struct_j::get()),
        )))
    } else {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructE);
        Err(samples::struct_e::get())
    }
}
