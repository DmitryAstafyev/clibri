use super::{hub, identification, producer::Control, protocol, Context};
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
pub async fn response<'c, E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: hub::filter::Filter,
    context: &Context,
    request: &protocol::GroupA::StructA,
    control: &Control<E, C>,
) -> Result<Response, protocol::GroupA::StructB> {
    let index = context
        .get_index(identification.uuid(), Alias::GroupAStructA)
        .await;
    if index == 1 {
        context
            .inc_stat(identification.uuid(), Alias::StructA)
            .await;
        context
            .inc_stat(identification.uuid(), Alias::StructD)
            .await;
        Ok(Response::RootA((
            samples::struct_a::get(),
            (vec![identification.uuid()], samples::struct_d::get()),
        )))
    } else if index == 2 {
        context
            .inc_stat(identification.uuid(), Alias::StructB)
            .await;
        Ok(Response::RootB(samples::struct_b::get()))
    } else {
        context
            .inc_stat(identification.uuid(), Alias::GroupAStructB)
            .await;
        Err(samples::group_a::struct_b::get())
    }
}
