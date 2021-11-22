use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::GroupB::GroupC::StructA,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.broadcast.groupb_groupc_structa += 1;
    context.broadcast.check();
}