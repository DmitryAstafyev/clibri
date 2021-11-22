
use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;
use crate::test::samples;

type BroadcastStructD = (Vec<Uuid>, protocol::StructD);

pub enum Response {    
    RootA(
    	(
    		protocol::StructA,
            BroadcastStructD,
    	)
    ),
    RootB(protocol::StructB),
}

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::GroupA::StructA,
    control: &Control<E, C>,
) -> Result<Response, protocol::GroupA::StructB> {
    let index = context.requests.groupa_structa(identification.uuid());
    if index == 1 {
        Ok(Response::RootA(
            (
                samples::struct_a::get(),
                (vec![identification.uuid()], samples::struct_d::get()),
            )
        ))
    } else if index == 2 {
        Ok(Response::RootB(samples::struct_b::get()))
    } else {
        Err(samples::group_a::struct_b::get())
    }
}
