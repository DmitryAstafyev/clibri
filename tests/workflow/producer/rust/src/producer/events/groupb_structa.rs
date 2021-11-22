use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;
use crate::test::samples;

type BroadcastStructA = (Vec<Uuid>, protocol::StructA);
type BroadcastStructB = (Vec<Uuid>, protocol::StructB);

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::GroupB::StructA,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(BroadcastStructA, BroadcastStructB), String> {
    Ok((
        (filter.all(), samples::struct_a::get()),
        (filter.all(), samples::struct_b::get()),
    ))
}