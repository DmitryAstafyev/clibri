use crate::{identification, producer::Control, protocol, Context};

#[allow(unused_variables)]
pub async fn response(
    context: &mut Context,
    request: &protocol::Users::Request,
    filter: identification::Filter,
    control: &Control,
) -> Result<protocol::Users::Response, protocol::Users::Err> {
    Ok(protocol::Users::Response { users: vec![] })
}
