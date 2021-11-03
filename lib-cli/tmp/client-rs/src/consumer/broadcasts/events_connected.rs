use super::{protocol, Consumer, Context};
use fiber::client;

pub async fn handler<E: client::Error>(
    event: protocol::Events::UserConnected,
    context: &mut Context,
    consumer: Consumer<E>,
) -> Result<(), String> {
    println!("{} is connected", event.username);
    Ok(())
    // Err(String::from(
    //     "Handler for Events::UserConnected isn't implemented yet",
    // ))
}
