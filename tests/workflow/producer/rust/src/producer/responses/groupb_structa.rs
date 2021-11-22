
use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;
use crate::test::samples;


pub enum Response {
    GroupBStructA(protocol::GroupB::StructA),
    GroupBGroupCStructA(protocol::GroupB::GroupC::StructA),
}

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::GroupB::StructA,
    control: &Control<E, C>,
) -> Result<Response, protocol::GroupB::GroupC::StructB> {
    let index = context.requests.groupb_structa(identification.uuid());
    if index == 1 {
        Ok(Response::GroupBStructA(samples::group_b::struct_a::get()))
    } else if index == 2 {
        Ok(Response::GroupBGroupCStructA(samples::group_b::group_c::struct_a::get()))
    } else {
        Err(samples::group_b::group_c::struct_b::get())
    }
}
