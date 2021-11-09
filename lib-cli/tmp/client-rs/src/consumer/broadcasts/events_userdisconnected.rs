use super::{protocol, Consumer, Context};
use fiber::client;

pub async fn handler<E: client::Error>(
    event: protocol::Events::UserDisconnected,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    println!("Events::UserDisconnected isn't implemented yet");
}