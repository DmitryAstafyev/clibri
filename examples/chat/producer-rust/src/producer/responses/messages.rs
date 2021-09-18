use super::{identification, producer::Control, protocol, Context};
use uuid::Uuid;

#[allow(unused_variables)]
pub async fn response(
    uuid: Uuid,
    context: &mut Context,
    request: &protocol::Messages::Request,
    filter: identification::Filter,
    control: &Control,
) -> Result<protocol::Messages::Response, protocol::Messages::Err> {
    Ok(protocol::Messages::Response {
        messages: context.get_messages(),
    })
}
