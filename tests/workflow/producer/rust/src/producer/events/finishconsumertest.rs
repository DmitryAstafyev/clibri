use super::{identification, producer::Control, protocol, scope::AnonymousScope, Context};
use crate::stat::Alias;
use clibri::server;
use std::str::FromStr;
use uuid::Uuid;

type BroadcastFinishConsumerTestBroadcast = (Vec<Uuid>, protocol::FinishConsumerTestBroadcast);

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    event: protocol::FinishConsumerTest,
    scope: &mut AnonymousScope<'_, E, C>,
) -> Result<BroadcastFinishConsumerTestBroadcast, String> {
    let uuid = match Uuid::from_str(&event.uuid) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Err(format!("Fail to parse uuid {}: {:?}", event.uuid, err));
        }
    };
    scope
        .context
        .inc_stat(uuid, Alias::FinishConsumerTestBroadcast);
    Ok((vec![uuid], protocol::FinishConsumerTestBroadcast {}))
}
