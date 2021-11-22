
use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use crate::test::samples;

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::StructEmpty,
    control: &Control<E, C>,
) -> Result<protocol::StructEmptyB, protocol::StructEmptyA> {
    let index = context.requests.structempty(identification.uuid());
    if index == 1 {
        Ok(samples::struct_empty_b::get())
    } else {
        if let Err(err) = control.events.structa(samples::struct_a::get()).await {
            panic!("Fail to emit control.events.structa: {}", err);
        }
        if let Err(err) = control.events.structb(samples::struct_b::get()).await {
            panic!("Fail to emit control.events.structb: {}", err);
        }
        if let Err(err) = control.events.groupb_structa(samples::group_b::struct_a::get()).await {
            panic!("Fail to emit control.events.groupb_structa: {}", err);
        }
        if let Err(err) = control.events.groupb_groupc_structa(samples::group_b::group_c::struct_a::get()).await {
            panic!("Fail to emit control.events.groupb_groupc_structa: {}", err);
        }
        if let Err(err) = control.events.groupd_structp(samples::group_d::struct_p::get()).await {
            panic!("Fail to emit control.events.groupd_structp: {}", err);
        }
        Err(samples::struct_empty_a::get())
    }
}
