use super::{identification, producer::Control, protocol, scope::AnonymousScope, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;
use std::str::FromStr;
use uuid::Uuid;

type BroadcastGroupAStructA = (Vec<Uuid>, protocol::GroupA::StructA);
type BroadcastGroupAStructB = (Vec<Uuid>, protocol::GroupA::StructB);
type BroadcastGroupBStructA = (Vec<Uuid>, protocol::GroupB::StructA);

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    event: protocol::Events::EventB,
    scope: &mut AnonymousScope<'_, E, C>,
) -> Result<
    (
        BroadcastGroupAStructA,
        BroadcastGroupAStructB,
        BroadcastGroupBStructA,
    ),
    String,
> {
    let uuid = match Uuid::from_str(&event.uuid) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Err(format!("Fail to parse uuid {}: {:?}", event.uuid, err));
        }
    };
    scope.context.inc_stat(uuid, Alias::GroupAStructA);
    scope.context.inc_stat(uuid, Alias::GroupAStructB);
    scope.context.inc_stat(uuid, Alias::GroupBStructA);
    Ok((
        (vec![uuid], samples::group_a::struct_a::get()),
        (vec![uuid], samples::group_a::struct_b::get()),
        (vec![uuid], samples::group_b::struct_a::get()),
    ))
}
