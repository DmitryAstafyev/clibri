use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::StructC,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.broadcast.structc += 1;
    context.broadcast.check();
}