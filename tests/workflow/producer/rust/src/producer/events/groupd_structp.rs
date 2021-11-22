use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use uuid::Uuid;
use crate::test::samples;

type BroadcastGroupBGroupCStructA = (Vec<Uuid>, protocol::GroupB::GroupC::StructA);
type BroadcastGroupBGroupCStructB = (Vec<Uuid>, protocol::GroupB::GroupC::StructB);

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::GroupD::StructP,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(BroadcastGroupBGroupCStructA, BroadcastGroupBGroupCStructB), String> {
    Ok((
        (filter.all(), samples::group_b::group_c::struct_a::get()),
        (filter.all(), samples::group_b::group_c::struct_b::get()),
    ))
}