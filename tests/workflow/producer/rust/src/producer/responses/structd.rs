use super::{identification, producer::Control, protocol, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;

#[allow(unused_variables)]
pub async fn response<'c, E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: &identification::Filter<'_>,
    context: &mut Context,
    request: &protocol::StructD,
    control: &Control<E, C>,
) -> Result<protocol::StructA, protocol::StructC> {
    let index = context.get_index(identification.uuid(), Alias::StructD);
    if index == 1 {
        context.inc_stat(identification.uuid(), Alias::StructA);
        Ok(samples::struct_a::get())
    } else {
        context.inc_stat(identification.uuid(), Alias::StructC);
        Err(samples::struct_c::get())
    }
}
