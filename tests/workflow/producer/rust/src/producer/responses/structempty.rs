use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::{stat::Alias, stop, test::samples};
use clibri::server;

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E>>(
    request: &protocol::StructEmpty,
    scope: &mut Scope<'_, E, C>,
) -> Result<protocol::StructEmptyB, protocol::StructEmptyA> {
    let index = scope
        .context
        .get_index(scope.identification.uuid(), Alias::StructEmpty);
    if index == 1 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructEmptyB);
        Ok(samples::struct_empty_b::get())
    } else {
        if let Err(err) = scope
            .control
            .events
            .eventa(protocol::EventA {
                uuid: scope.identification.uuid().to_string(),
                field_a: samples::struct_b::get(),
                field_b: samples::struct_c::get(),
            })
            .await
        {
            stop!("Fail to emit control.events.eventa: {}", err);
        }
        if let Err(err) = scope
            .control
            .events
            .eventb(protocol::EventB {
                uuid: scope.identification.uuid().to_string(),
                field_a: samples::struct_c::get(),
            })
            .await
        {
            stop!("Fail to emit control.events.eventb: {}", err);
        }
        if let Err(err) = scope
            .control
            .events
            .events_eventa(protocol::Events::EventA {
                uuid: scope.identification.uuid().to_string(),
                field_a: samples::struct_a::get(),
                field_b: samples::struct_b::get(),
            })
            .await
        {
            stop!("Fail to emit control.events.events_eventa: {}", err);
        }
        if let Err(err) = scope
            .control
            .events
            .events_eventb(protocol::Events::EventB {
                uuid: scope.identification.uuid().to_string(),
                field_a: samples::group_a::struct_a::get(),
                field_b: samples::group_a::struct_b::get(),
                field_c: samples::group_b::struct_a::get(),
            })
            .await
        {
            stop!("Fail to emit control.events.events_eventb: {}", err);
        }
        if let Err(err) = scope
            .control
            .events
            .events_sub_eventa(protocol::Events::Sub::EventA {
                uuid: scope.identification.uuid().to_string(),
                field_a: samples::group_b::group_c::struct_a::get(),
                field_b: samples::group_b::group_c::struct_b::get(),
            })
            .await
        {
            stop!("Fail to emit control.events.events_sub_eventa: {}", err);
        }
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructEmptyA);
        Err(samples::struct_empty_a::get())
    }
}
