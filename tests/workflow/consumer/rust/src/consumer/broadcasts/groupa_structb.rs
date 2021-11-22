use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::GroupA::StructB,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.broadcast.groupa_structb += 1;
    context.broadcast.check();
}