use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::FinishConsumerTestBroadcast,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.finish.cancel();
}