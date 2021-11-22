use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::FinishConsumerTestUuidBroadcast,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    println!("FinishConsumerTestUuidBroadcast isn't implemented yet");
}