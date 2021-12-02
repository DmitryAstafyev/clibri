use super::{Consumer, ConsumerError, Context};
use crate::stat::Alias;
use clibri::client;

pub async fn handler<E: client::Error>(
    error: Option<ConsumerError<E>>,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.inc_stat(Alias::Shutdown);
}
