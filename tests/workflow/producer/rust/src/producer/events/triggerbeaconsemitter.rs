use super::{identification, producer::Control, protocol, scope::AnonymousScope, Context};
use clibri::server;
use std::str::FromStr;
use uuid::Uuid;

type BroadcastTriggerBeacons = (Vec<Uuid>, protocol::TriggerBeacons);

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    event: protocol::TriggerBeaconsEmitter,
    scope: &mut AnonymousScope<'_, E, C>,
) -> Result<BroadcastTriggerBeacons, String> {
    let uuid = match Uuid::from_str(&event.uuid) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Err(format!("Fail to parse uuid {}: {:?}", event.uuid, err));
        }
    };
    Ok((vec![uuid], protocol::TriggerBeacons {}))
}
