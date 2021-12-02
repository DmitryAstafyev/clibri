use super::{controller, protocol, Consumer, Context};
use crate::stat::Alias;
use clibri::client;

pub async fn handler<E: client::Error>(context: &mut Context, mut consumer: Consumer<E>) {
    context.connected.cancel();
    context.inc_stat(Alias::Connected);
}
