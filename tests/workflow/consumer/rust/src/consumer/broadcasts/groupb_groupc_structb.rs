use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::GroupB::GroupC::StructB,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.broadcast.groupb_groupc_structb += 1;
    context.broadcast.check();
}