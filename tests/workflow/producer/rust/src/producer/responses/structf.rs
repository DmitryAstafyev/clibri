use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::{stat::Alias, stop, test::samples};
use clibri::server;

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E>>(
    request: &protocol::StructF,
    scope: &mut Scope<'_, E, C>,
) -> Result<protocol::StructF, protocol::StructE> {
    let index = scope
        .context
        .get_index(scope.identification.uuid(), Alias::StructF);
    if index == 1 {
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructF);
        Ok(samples::struct_f::get())
    } else {
        if let Err(err) = scope
            .control
            .events
            .triggerbeaconsemitter(protocol::TriggerBeaconsEmitter {
                uuid: scope.identification.uuid().to_string(),
            })
            .await
        {
            stop!("Fail to emit control.events.structuuid: {}", err);
        }
        scope
            .context
            .inc_stat(scope.identification.uuid(), Alias::StructE);
        Err(samples::struct_e::get())
    }
}
