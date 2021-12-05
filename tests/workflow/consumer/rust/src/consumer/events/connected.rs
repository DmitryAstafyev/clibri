use super::{Consumer, Context};
use crate::stat::Alias;
use clibri::client;

pub async fn handler<E: client::Error>(context: &mut Context, consumer: Consumer<E>) {
    context.connected.cancel();
    context.inc_stat(Alias::Connected);
}
