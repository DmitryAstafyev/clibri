use super::{identification, producer::Control, protocol, Context};
use crate::test::samples;
use clibri::server;
use uuid::Uuid;

type BroadcastStructC = (Vec<Uuid>, protocol::StructC);

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::StructB,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<BroadcastStructC, String> {
    Ok((filter.all(), samples::struct_c::get()))
}
