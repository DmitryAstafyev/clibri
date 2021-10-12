
use super::{identification, producer::Control, protocol, Context};
use fiber::server;

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::Users::Request,
    control: &Control<E, C>,
) -> Result<protocol::Users::Response, protocol::Users::Err> {
    panic!("Handler for protocol::Users::Request isn't implemented");
}
