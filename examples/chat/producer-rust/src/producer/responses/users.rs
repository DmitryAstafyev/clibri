use super::{identification, producer::Control, protocol, Context};

#[allow(unused_variables)]
pub async fn response(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::Users::Request,
    control: &Control,
) -> Result<protocol::Users::Response, protocol::Users::Err> {
    Ok(protocol::Users::Response {
        users: context.get_users(),
    })
}
