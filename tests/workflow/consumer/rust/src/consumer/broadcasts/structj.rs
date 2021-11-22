use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::StructJ,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.broadcast.structj += 1;
    context.broadcast.check();
}