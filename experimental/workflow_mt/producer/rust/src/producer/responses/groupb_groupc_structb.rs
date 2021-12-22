use super::{hub, identification, producer::Control, protocol, Context};
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
pub async fn response<'c, E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: hub::filter::Filter,
    context: &Context,
    request: &protocol::GroupB::GroupC::StructB,
    control: &Control<E, C>,
) -> Result<Response, protocol::GroupB::GroupC::StructA> {
    let index = context
        .get_index(identification.uuid(), Alias::GroupBGroupCStructB)
        .await;
    if index == 1 {
        context
            .inc_stat(identification.uuid(), Alias::StructB)
            .await;
        context
            .inc_stat(identification.uuid(), Alias::StructD)
            .await;
        context
            .inc_stat(identification.uuid(), Alias::StructF)
            .await;
        Ok(Response::CaseB((
            samples::struct_b::get(),
            (vec![identification.uuid()], samples::struct_d::get()),
            (vec![identification.uuid()], samples::struct_f::get()),
        )))
    } else if index == 2 {
        context
            .inc_stat(identification.uuid(), Alias::StructC)
            .await;
        Ok(Response::CaseC(samples::struct_c::get()))
    } else if index == 3 {
        context
            .inc_stat(identification.uuid(), Alias::StructD)
            .await;
        context
            .inc_stat(identification.uuid(), Alias::StructJ)
            .await;
        Ok(Response::CaseD((
            samples::struct_d::get(),
            (vec![identification.uuid()], samples::struct_j::get()),
        )))
    } else {
        context
            .inc_stat(identification.uuid(), Alias::GroupBGroupCStructA)
            .await;
        Err(samples::group_b::group_c::struct_a::get())
    }
}
