use super::{protocol, Consumer, Context};
use crate::stat::Alias;
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::GroupB::GroupC::StructB,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.inc_stat(Alias::GroupBGroupCStructB);
}
