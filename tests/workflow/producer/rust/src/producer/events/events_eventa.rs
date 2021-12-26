use super::{identification, producer::Control, protocol, scope::AnonymousScope, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;
use std::str::FromStr;
use uuid::Uuid;

type BroadcastStructA = (Vec<Uuid>, protocol::StructA);
type BroadcastStructB = (Vec<Uuid>, protocol::StructB);

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    event: protocol::Events::EventA,
    scope: &mut AnonymousScope<'_, E, C>,
) -> Result<(BroadcastStructA, BroadcastStructB), String> {
    let uuid = match Uuid::from_str(&event.uuid) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Err(format!("Fail to parse uuid {}: {:?}", event.uuid, err));
        }
    };
    scope.context.inc_stat(uuid, Alias::StructA);
    scope.context.inc_stat(uuid, Alias::StructB);
    Ok((
        (vec![uuid], samples::struct_a::get()),
        (vec![uuid], samples::struct_b::get()),
    ))
}
