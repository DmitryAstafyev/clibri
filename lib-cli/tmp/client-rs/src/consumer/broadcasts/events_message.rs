use super::{protocol, Consumer, Context};

pub async fn handler<E: std::error::Error + Clone>(
    event: protocol::Events::Message,
    context: &mut Context,
    consumer: Consumer<E>,
) -> Result<(), String> {
    println!("[{}] {}: {}", event.timestamp, event.user, event.message);
    Ok(())
    // Err(String::from(
    //     "Handler for Events::Message isn't implemented yet",
    // ))
}
