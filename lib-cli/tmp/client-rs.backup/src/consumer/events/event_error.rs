use super::{Consumer, ConsumerError, Context};
use fiber::client;

pub async fn handler<E: client::Error>(
    error: ConsumerError<E>,
    context: &mut Context,
    consumer: Consumer<E>,
) {
}
