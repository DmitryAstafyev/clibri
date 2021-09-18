use super::{identification, producer::Control, protocol, Context};
use uuid::Uuid;

#[allow(unused_variables)]
pub async fn response(
    uuid: Uuid,
    context: &mut Context,
    request: &protocol::Users::Request,
    filter: identification::Filter,
    control: &Control,
) -> Result<protocol::Users::Response, protocol::Users::Err> {
    Ok(protocol::Users::Response {
        users: context.get_users(),
    })
}
