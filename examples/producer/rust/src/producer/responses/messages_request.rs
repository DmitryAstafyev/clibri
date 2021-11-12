use super::{identification, producer::Control, protocol, Context};
use fiber::server;

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::Messages::Request,
    control: &Control<E, C>,
) -> Result<protocol::Messages::Response, protocol::Messages::Err> {
    Ok(protocol::Messages::Response {
        messages: context.get_messages(),
    })
}
