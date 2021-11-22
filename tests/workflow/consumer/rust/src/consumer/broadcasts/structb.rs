use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::StructB,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.broadcast.structb += 1;
    context.broadcast.check();
}