use super::{identification, producer::Control, protocol, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;

#[allow(unused_variables)]
pub async fn response<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::StructF,
    control: &Control<E, C>,
) -> Result<protocol::StructF, protocol::StructE> {
    let index = context.get_index(identification.uuid(), Alias::StructF);
    if index == 1 {
        context.inc_stat(identification.uuid(), Alias::StructF);
        Ok(samples::struct_f::get())
    } else {
        if let Err(err) = control
            .events
            .triggerbeaconsemitter(protocol::TriggerBeaconsEmitter {
                uuid: identification.uuid().to_string(),
            })
            .await
        {
            panic!("Fail to emit control.events.structuuid: {}", err);
        }
        context.inc_stat(identification.uuid(), Alias::StructE);
        Err(samples::struct_e::get())
    }
}
