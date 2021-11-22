
use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use crate::test::samples;

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::StructD,
    control: &Control<E, C>,
) -> Result<protocol::StructA, protocol::StructC> {
    let index = context.requests.structd(identification.uuid());
    if index == 1 {
        Ok(samples::struct_a::get())
    } else {
        Err(samples::struct_c::get())
    }
}
