use super::{hub, identification, producer::Control, protocol, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;
use uuid::Uuid;

type BroadcastGroupBGroupCStructB = (Vec<Uuid>, protocol::GroupB::GroupC::StructB);

pub enum Response {
    GroupBStructA((protocol::GroupB::StructA, BroadcastGroupBGroupCStructB)),
    GroupBGroupCStructA(protocol::GroupB::GroupC::StructA),
}

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: hub::filter::Filter,
    context: &Context,
    request: &protocol::GroupA::StructB,
    control: &Control<E, C>,
) -> Result<Response, protocol::GroupA::StructB> {
    let index = context
        .get_index(identification.uuid(), Alias::GroupAStructB)
        .await;
    if index == 1 {
        context
            .inc_stat(identification.uuid(), Alias::GroupBStructA)
            .await;
        context
            .inc_stat(identification.uuid(), Alias::GroupBGroupCStructB)
            .await;
        Ok(Response::GroupBStructA((
            samples::group_b::struct_a::get(),
            (
                vec![identification.uuid()],
                samples::group_b::group_c::struct_b::get(),
            ),
        )))
    } else if index == 2 {
        context
            .inc_stat(identification.uuid(), Alias::GroupBGroupCStructA)
            .await;
        Ok(Response::GroupBGroupCStructA(
            samples::group_b::group_c::struct_a::get(),
        ))
    } else {
        context
            .inc_stat(identification.uuid(), Alias::GroupAStructB)
            .await;
        Err(samples::group_a::struct_b::get())
    }
}
