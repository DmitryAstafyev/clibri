use super::{protocol, Consumer, Context};

pub async fn handler<E: std::error::Error + Clone>(
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
