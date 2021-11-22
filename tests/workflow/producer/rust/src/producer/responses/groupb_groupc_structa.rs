
use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use crate::test::samples;

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::GroupB::GroupC::StructA,
    control: &Control<E, C>,
) -> Result<protocol::GroupB::GroupC::StructB, protocol::GroupA::StructB> {
    let index = context.requests.groupb_groupc_structa(identification.uuid());
    if index == 1 {
        Ok(samples::group_b::group_c::struct_b::get())
    } else {
        Err(samples::group_a::struct_b::get())
    }
}
