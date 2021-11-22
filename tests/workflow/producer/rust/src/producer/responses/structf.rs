
use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use crate::test::samples;

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::StructF,
    control: &Control<E, C>,
) -> Result<protocol::StructF, protocol::StructE> {
    let index = context.requests.structf(identification.uuid());
    if index == 1 {
        Ok(samples::struct_f::get())
    } else {
        if let Err(err) = control.events.structuuid(protocol::StructUuid { uuid: identification.uuid().to_string()}).await {
            panic!("Fail to emit control.events.structuuid: {}", err);
        }
        Err(samples::struct_e::get())
    }
}
