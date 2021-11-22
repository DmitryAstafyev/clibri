use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::GroupA::StructA,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.broadcast.groupa_structa += 1;
    context.broadcast.check();
}