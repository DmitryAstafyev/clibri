use super::{protocol, Consumer, Context};

pub async fn handler<E: std::error::Error + Clone>(
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
