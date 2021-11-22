
use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;
use crate::test::samples;

pub enum Response {    
    CaseB(protocol::StructB),
    CaseF(protocol::StructF),
    CaseD(protocol::StructD),
}

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::StructC,
    control: &Control<E, C>,
) -> Result<Response, protocol::StructE> {
    let index = context.requests.structc(identification.uuid());
    if index == 1 {
        Ok(Response::CaseB(samples::struct_b::get()))
    } else if index == 2 {
        Ok(Response::CaseF(samples::struct_f::get()))
    } else if index == 3 {
        Ok(Response::CaseD(samples::struct_d::get()))
    } else {
        Err(samples::struct_e::get())
    }
}
