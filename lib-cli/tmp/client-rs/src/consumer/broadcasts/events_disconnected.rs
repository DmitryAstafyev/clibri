use super::{protocol, Consumer, Context};

pub async fn handler<E: std::error::Error>(
    event: protocol::Events::UserDisconnected,
    context: &mut Context,
    consumer: &mut Consumer<E>,
) -> Result<(), String> {
    Err(String::from(
        "Handler for Events::UserDisconnected isn't implemented yet",
    ))
}
