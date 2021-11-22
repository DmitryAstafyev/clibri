use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;
use crate::test::samples;

type BroadcastStructB = (Vec<Uuid>, protocol::StructB);
type BroadcastStructC = (Vec<Uuid>, protocol::StructC);

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::StructA,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(BroadcastStructB, BroadcastStructC), String> {
    Ok((
        (filter.all(), samples::struct_b::get()),
        (filter.all(), samples::struct_c::get()),
    ))}