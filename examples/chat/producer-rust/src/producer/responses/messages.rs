use super::{identification, producer::Control, protocol, Context};
use uuid::Uuid;

#[allow(unused_variables)]
pub async fn response(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::Messages::Request,
    control: &Control,
) -> Result<protocol::Messages::Response, protocol::Messages::Err> {
    Ok(protocol::Messages::Response {
        messages: context.get_messages(),
    })
}
