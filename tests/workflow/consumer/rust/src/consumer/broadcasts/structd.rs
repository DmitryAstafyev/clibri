use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::StructD,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.broadcast.structd += 1;
    context.broadcast.check();
}