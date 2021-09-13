use crate::{identification, producer::Control, protocol, Context};

#[allow(unused_variables)]
pub async fn response(
    context: &mut Context,
    request: &protocol::Messages::Request,
    filter: identification::Filter,
    control: &Control,
) -> Result<protocol::Messages::Response, protocol::Messages::Err> {
    Ok(protocol::Messages::Response { messages: vec![] })
}
