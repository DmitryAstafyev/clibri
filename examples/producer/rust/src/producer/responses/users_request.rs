use super::{identification, producer::Control, protocol, scope::Scope, Context};
use clibri::server;

#[allow(unused_variables)]
pub async fn response<E: server::Error, C: server::Control<E>>(
    request: &protocol::Users::Request,
    scope: &mut Scope<'_, E, C>,
) -> Result<protocol::Users::Response, protocol::Users::Err> {
    Ok(protocol::Users::Response {
        users: scope.context.get_users(),
    })
}
