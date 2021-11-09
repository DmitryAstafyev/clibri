use super::{protocol, Consumer, Context};
use fiber::client;

pub async fn handler<E: client::Error>(
    event: protocol::Events::UserConnected,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    println!("{} isn't implemented yet");
}