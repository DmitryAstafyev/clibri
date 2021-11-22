use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::StructA,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.broadcast.structa += 1;
    context.broadcast.check();
}