use super::{identification, producer::Control, protocol, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;

#[allow(unused_variables)]
pub async fn response<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::StructEmpty,
    control: &Control<E, C>,
) -> Result<protocol::StructEmptyB, protocol::StructEmptyA> {
    let index = context.get_index(identification.uuid(), Alias::StructEmpty);
    if index == 1 {
        context.inc_stat(identification.uuid(), Alias::StructEmptyB);
        Ok(samples::struct_empty_b::get())
    } else {
        // if let Err(err) = control.events.structa(samples::struct_a::get()).await {
        //     panic!("Fail to emit control.events.structa: {}", err);
        // }
        // if let Err(err) = control.events.structb(samples::struct_b::get()).await {
        //     panic!("Fail to emit control.events.structb: {}", err);
        // }
        // if let Err(err) = control
        //     .events
        //     .groupb_structa(samples::group_b::struct_a::get())
        //     .await
        // {
        //     panic!("Fail to emit control.events.groupb_structa: {}", err);
        // }
        // if let Err(err) = control
        //     .events
        //     .groupb_groupc_structa(samples::group_b::group_c::struct_a::get())
        //     .await
        // {
        //     panic!("Fail to emit control.events.groupb_groupc_structa: {}", err);
        // }
        // if let Err(err) = control
        //     .events
        //     .groupd_structp(samples::group_d::struct_p::get())
        //     .await
        // {
        //     panic!("Fail to emit control.events.groupd_structp: {}", err);
        // }
        context.inc_stat(identification.uuid(), Alias::StructEmptyA);
        Err(samples::struct_empty_a::get())
    }
}
