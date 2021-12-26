use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;
use uuid::Uuid;

pub enum Response {
    CaseB(protocol::StructB),
    CaseF(protocol::StructF),
    CaseD(protocol::StructD),
}

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E>>(
    request: &protocol::StructC,
    scope: &mut Scope<'_, E, C>,
) -> Result<Response, protocol::StructE> {
    let index = scope
        .context
        .get_index(scope.identification.uuid(), Alias::StructC);
    if index == 1 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructB);
        Ok(Response::CaseB(samples::struct_b::get()))
    } else if index == 2 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructF);
        Ok(Response::CaseF(samples::struct_f::get()))
    } else if index == 3 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructD);
        Ok(Response::CaseD(samples::struct_d::get()))
    } else {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructE);
        Err(samples::struct_e::get())
    }
}
