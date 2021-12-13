use super::{Consumer, ConsumerError, Context};
use crate::{stat::Alias, stop};
use clibri::client;

pub async fn handler<E: client::Error>(
    error: ConsumerError<E>,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    stop!("{:?}", error);
    context.inc_stat(Alias::Error);
}
