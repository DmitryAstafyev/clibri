use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;
use std::str::FromStr;

type BroadcastStructEmptyB = (Vec<Uuid>, protocol::StructEmptyB);

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::StructUuid,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<BroadcastStructEmptyB, String> {
    let uuid = match Uuid::from_str(&event.uuid) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Err(format!("Fail to parse uuid {}: {:?}", event.uuid, err));
        }
    };
    Ok((vec![uuid], protocol::StructEmptyB {}))
}