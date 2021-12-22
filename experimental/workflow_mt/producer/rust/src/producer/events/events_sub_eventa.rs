use super::{hub, identification, producer::Control, protocol, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;
use std::str::FromStr;
use uuid::Uuid;

type BroadcastGroupBGroupCStructA = (Vec<Uuid>, protocol::GroupB::GroupC::StructA);
type BroadcastGroupBGroupCStructB = (Vec<Uuid>, protocol::GroupB::GroupC::StructB);

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::Events::Sub::EventA,
    filter: hub::filter::Filter,
    context: &Context,
    control: &Control<E, C>,
) -> Result<(BroadcastGroupBGroupCStructA, BroadcastGroupBGroupCStructB), String> {
    let uuid = match Uuid::from_str(&event.uuid) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Err(format!("Fail to parse uuid {}: {:?}", event.uuid, err));
        }
    };
    context.inc_stat(uuid, Alias::GroupBGroupCStructA).await;
    context.inc_stat(uuid, Alias::GroupBGroupCStructB).await;
    Ok((
        (vec![uuid], samples::group_b::group_c::struct_a::get()),
        (vec![uuid], samples::group_b::group_c::struct_b::get()),
    ))
}
