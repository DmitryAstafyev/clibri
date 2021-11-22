use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::StructF,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.broadcast.structf += 1;
    context.broadcast.check();
}