use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::Events::UserDisconnected,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    println!("{} is disconnected", event.username);
}
