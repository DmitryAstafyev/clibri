use super::{protocol, Consumer, Context};
use fiber::client;

pub async fn handler<E: client::Error>(
    event: protocol::Events::UserDisconnected,
    context: &mut Context,
    consumer: Consumer<E>,
) -> Result<(), String> {
    println!("{} is disconnected", event.username);
    Ok(())

    // Err(String::from(
    //     "Handler for Events::UserDisconnected isn't implemented yet",
    // ))
}
