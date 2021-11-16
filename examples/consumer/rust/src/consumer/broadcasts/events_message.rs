use super::{protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::Events::Message,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    println!(
        "[{}] {}: {}",
        context.get_localtime(event.timestamp),
        event.user,
        event.message.trim()
    );
}
