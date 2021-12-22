use super::{hub, identification, producer::Control, protocol, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;
use std::str::FromStr;
use uuid::Uuid;

type BroadcastStructB = (Vec<Uuid>, protocol::StructB);
type BroadcastStructC = (Vec<Uuid>, protocol::StructC);

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::EventA,
    filter: hub::filter::Filter,
    context: &Context,
    control: &Control<E, C>,
) -> Result<(BroadcastStructB, BroadcastStructC), String> {
    let uuid = match Uuid::from_str(&event.uuid) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Err(format!("Fail to parse uuid {}: {:?}", event.uuid, err));
        }
    };
    context.inc_stat(uuid, Alias::StructB).await;
    context.inc_stat(uuid, Alias::StructC).await;
    Ok((
        (vec![uuid], samples::struct_b::get()),
        (vec![uuid], samples::struct_c::get()),
    ))
}
