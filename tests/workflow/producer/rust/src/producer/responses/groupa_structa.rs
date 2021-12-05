use super::{identification, producer::Control, protocol, Context};
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
pub async fn response<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::GroupA::StructA,
    control: &Control<E, C>,
) -> Result<Response, protocol::GroupA::StructB> {
    let index = context.get_index(identification.uuid(), Alias::GroupAStructA);
    if index == 1 {
        context.inc_stat(identification.uuid(), Alias::StructA);
        context.inc_stat(identification.uuid(), Alias::StructD);
        Ok(Response::RootA((
            samples::struct_a::get(),
            (vec![identification.uuid()], samples::struct_d::get()),
        )))
    } else if index == 2 {
        context.inc_stat(identification.uuid(), Alias::StructB);
        Ok(Response::RootB(samples::struct_b::get()))
    } else {
        context.inc_stat(identification.uuid(), Alias::GroupAStructB);
        Err(samples::group_a::struct_b::get())
    }
}
