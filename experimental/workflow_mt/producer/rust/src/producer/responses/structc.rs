use super::{hub, identification, producer::Control, protocol, Context};
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
pub async fn response<'c, E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: hub::filter::Filter,
    context: &Context,
    request: &protocol::StructC,
    control: &Control<E, C>,
) -> Result<Response, protocol::StructE> {
    let index = context
        .get_index(identification.uuid(), Alias::StructC)
        .await;
    if index == 1 {
        context
            .inc_stat(identification.uuid(), Alias::StructB)
            .await;
        Ok(Response::CaseB(samples::struct_b::get()))
    } else if index == 2 {
        context
            .inc_stat(identification.uuid(), Alias::StructF)
            .await;
        Ok(Response::CaseF(samples::struct_f::get()))
    } else if index == 3 {
        context
            .inc_stat(identification.uuid(), Alias::StructD)
            .await;
        Ok(Response::CaseD(samples::struct_d::get()))
    } else {
        context
            .inc_stat(identification.uuid(), Alias::StructE)
            .await;
        Err(samples::struct_e::get())
    }
}
