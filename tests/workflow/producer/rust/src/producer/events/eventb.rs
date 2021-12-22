use super::{identification, producer::Control, protocol, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;
use std::str::FromStr;
use uuid::Uuid;

type BroadcastStructC = (Vec<Uuid>, protocol::StructC);

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::EventB,
    filter: &identification::Filter<'_>,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<BroadcastStructC, String> {
    let uuid = match Uuid::from_str(&event.uuid) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Err(format!("Fail to parse uuid {}: {:?}", event.uuid, err));
        }
    };
    context.inc_stat(uuid, Alias::StructC);
    Ok((vec![uuid], samples::struct_c::get()))
}
