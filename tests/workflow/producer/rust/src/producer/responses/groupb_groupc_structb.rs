
use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;
use crate::test::samples;

type BroadcastStructD = (Vec<Uuid>, protocol::StructD);
type BroadcastStructF = (Vec<Uuid>, protocol::StructF);
type BroadcastStructJ = (Vec<Uuid>, protocol::StructJ);

pub enum Response {    
    CaseB(
    	(
    		protocol::StructB,
            BroadcastStructD,
            BroadcastStructF,
    	)
    ),
    CaseC(protocol::StructC),
    
    CaseD(
    	(
    		protocol::StructD,
            BroadcastStructJ,
    	)
    ),
}

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::GroupB::GroupC::StructB,
    control: &Control<E, C>,
) -> Result<Response, protocol::GroupB::GroupC::StructA> {
    let index = context.requests.groupb_groupc_structb(identification.uuid());
    if index == 1 {
        Ok(Response::CaseB(
            (
                samples::struct_b::get(),
                (vec![identification.uuid()], samples::struct_d::get()),
                (vec![identification.uuid()], samples::struct_f::get())
            )
        ))
    } else if index == 2 {
        Ok(Response::CaseC(samples::struct_c::get()))
    } else if index == 3 {
        Ok(Response::CaseD(
            (
                samples::struct_d::get(),
                (vec![identification.uuid()], samples::struct_j::get())

            )
        ))
    } else {
        Err(samples::group_b::group_c::struct_a::get())
    }
}
