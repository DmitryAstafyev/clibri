use super::{hub, identification, producer::Control, protocol, Context};
use crate::{stat::Alias, stop, test::samples};
use clibri::server;

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: hub::filter::Filter,
    context: &Context,
    request: &protocol::StructF,
    control: &Control<E, C>,
) -> Result<protocol::StructF, protocol::StructE> {
    let index = context
        .get_index(identification.uuid(), Alias::StructF)
        .await;
    if index == 1 {
        context
            .inc_stat(identification.uuid(), Alias::StructF)
            .await;
        Ok(samples::struct_f::get())
    } else {
        if let Err(err) = control
            .events
            .triggerbeaconsemitter(protocol::TriggerBeaconsEmitter {
                uuid: identification.uuid().to_string(),
            })
            .await
        {
            stop!("Fail to emit control.events.structuuid: {}", err);
        }
        context
            .inc_stat(identification.uuid(), Alias::StructE)
            .await;
        Err(samples::struct_e::get())
    }
}
