use super::{hub, identification, producer::Control, protocol, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: hub::filter::Filter,
    context: &Context,
    request: &protocol::GroupB::GroupC::StructA,
    control: &Control<E, C>,
) -> Result<protocol::GroupB::GroupC::StructB, protocol::GroupA::StructB> {
    let index = context
        .get_index(identification.uuid(), Alias::GroupBGroupCStructA)
        .await;
    if index == 1 {
        context
            .inc_stat(identification.uuid(), Alias::GroupBGroupCStructB)
            .await;
        Ok(samples::group_b::group_c::struct_b::get())
    } else {
        context
            .inc_stat(identification.uuid(), Alias::GroupAStructB)
            .await;
        Err(samples::group_a::struct_b::get())
    }
}
