use super::{identification, producer::Control, protocol, Context};
use crate::{stat::Alias, stop, test::samples};
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
        if let Err(err) = control
            .events
            .eventa(protocol::EventA {
                uuid: identification.uuid().to_string(),
                field_a: samples::struct_b::get(),
                field_b: samples::struct_c::get(),
            })
            .await
        {
            stop!("Fail to emit control.events.eventa: {}", err);
        }
        if let Err(err) = control
            .events
            .eventb(protocol::EventB {
                uuid: identification.uuid().to_string(),
                field_a: samples::struct_c::get(),
            })
            .await
        {
            stop!("Fail to emit control.events.eventb: {}", err);
        }
        if let Err(err) = control
            .events
            .events_eventa(protocol::Events::EventA {
                uuid: identification.uuid().to_string(),
                field_a: samples::struct_a::get(),
                field_b: samples::struct_b::get(),
            })
            .await
        {
            stop!("Fail to emit control.events.events_eventa: {}", err);
        }
        if let Err(err) = control
            .events
            .events_eventb(protocol::Events::EventB {
                uuid: identification.uuid().to_string(),
                field_a: samples::group_a::struct_a::get(),
                field_b: samples::group_a::struct_b::get(),
                field_c: samples::group_b::struct_a::get(),
            })
            .await
        {
            stop!("Fail to emit control.events.events_eventb: {}", err);
        }
        if let Err(err) = control
            .events
            .events_sub_eventa(protocol::Events::Sub::EventA {
                uuid: identification.uuid().to_string(),
                field_a: samples::group_b::group_c::struct_a::get(),
                field_b: samples::group_b::group_c::struct_b::get(),
            })
            .await
        {
            stop!("Fail to emit control.events.events_sub_eventa: {}", err);
        }
        context.inc_stat(identification.uuid(), Alias::StructEmptyA);
        Err(samples::struct_empty_a::get())
    }
}
