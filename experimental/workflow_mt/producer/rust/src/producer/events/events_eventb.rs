use super::{hub, identification, producer::Control, protocol, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;
use std::str::FromStr;
use uuid::Uuid;

type BroadcastGroupAStructA = (Vec<Uuid>, protocol::GroupA::StructA);
type BroadcastGroupAStructB = (Vec<Uuid>, protocol::GroupA::StructB);
type BroadcastGroupBStructA = (Vec<Uuid>, protocol::GroupB::StructA);

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::Events::EventB,
    filter: hub::filter::Filter,
    context: &Context,
    control: &Control<E, C>,
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
    context.inc_stat(uuid, Alias::GroupAStructA).await;
    context.inc_stat(uuid, Alias::GroupAStructB).await;
    context.inc_stat(uuid, Alias::GroupBStructA).await;
    Ok((
        (vec![uuid], samples::group_a::struct_a::get()),
        (vec![uuid], samples::group_a::struct_b::get()),
        (vec![uuid], samples::group_b::struct_a::get()),
    ))
}
