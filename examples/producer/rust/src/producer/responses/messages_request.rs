use super::{identification, producer::Control, protocol, scope::Scope, Context};
use clibri::server;

#[allow(unused_variables)]
pub async fn response<E: server::Error, C: server::Control<E>>(
    request: &protocol::Messages::Request,
    scope: &mut Scope<'_, E, C>,
) -> Result<protocol::Messages::Response, protocol::Messages::Err> {
    Ok(protocol::Messages::Response {
        messages: scope.context.get_messages(),
    })
}
