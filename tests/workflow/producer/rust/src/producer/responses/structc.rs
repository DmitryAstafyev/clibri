use super::{identification, producer::Control, protocol, Context};
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
pub async fn response<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::StructC,
    control: &Control<E, C>,
) -> Result<Response, protocol::StructE> {
    let index = context.get_index(identification.uuid(), Alias::StructC);
    if index == 1 {
        context.inc_stat(identification.uuid(), Alias::StructB);
        Ok(Response::CaseB(samples::struct_b::get()))
    } else if index == 2 {
        context.inc_stat(identification.uuid(), Alias::StructF);
        Ok(Response::CaseF(samples::struct_f::get()))
    } else if index == 3 {
        context.inc_stat(identification.uuid(), Alias::StructD);
        Ok(Response::CaseD(samples::struct_d::get()))
    } else {
        context.inc_stat(identification.uuid(), Alias::StructE);
        Err(samples::struct_e::get())
    }
}
