
use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;
use crate::test::samples;

type BroadcastGroupBGroupCStructB = (Vec<Uuid>, protocol::GroupB::GroupC::StructB);

pub enum Response {    
    GroupBStructA(
    	(
    		protocol::GroupB::StructA,
            BroadcastGroupBGroupCStructB,
    	)
    ),
    GroupBGroupCStructA(protocol::GroupB::GroupC::StructA),
}

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::GroupA::StructB,
    control: &Control<E, C>,
) -> Result<Response, protocol::GroupA::StructB> {
    let index = context.requests.groupa_structb(identification.uuid());
    if index == 1 {
        Ok(Response::GroupBStructA(
            (
                samples::group_b::struct_a::get(),
                (vec![identification.uuid()], samples::group_b::group_c::struct_b::get()),
            )
        ))
    } else if index == 2 {
        Ok(Response::GroupBGroupCStructA(samples::group_b::group_c::struct_a::get()))
    } else {
        Err(samples::group_a::struct_b::get())
    }
}
