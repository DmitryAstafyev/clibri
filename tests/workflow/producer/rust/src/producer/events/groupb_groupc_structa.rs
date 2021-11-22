use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;
use crate::test::samples;

type BroadcastGroupAStructA = (Vec<Uuid>, protocol::GroupA::StructA);
type BroadcastGroupAStructB = (Vec<Uuid>, protocol::GroupA::StructB);
type BroadcastGroupBStructA = (Vec<Uuid>, protocol::GroupB::StructA);

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::GroupB::GroupC::StructA,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(BroadcastGroupAStructA, BroadcastGroupAStructB, BroadcastGroupBStructA), String> {
    Ok((
        (filter.all(), samples::group_a::struct_a::get()),
        (filter.all(), samples::group_a::struct_b::get()),
        (filter.all(), samples::group_b::struct_a::get()),
    ))
}