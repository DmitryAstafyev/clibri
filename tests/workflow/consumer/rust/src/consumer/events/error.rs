use super::{Consumer, ConsumerError, Context};
use crate::stat::Alias;
use clibri::client;

pub async fn handler<E: client::Error>(
    error: ConsumerError<E>,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    eprintln!("{:?}", error);
    context.inc_stat(Alias::Error);
}
