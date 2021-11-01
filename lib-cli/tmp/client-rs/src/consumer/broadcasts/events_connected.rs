use super::{protocol, Consumer, Context};

pub async fn handler<E: std::error::Error>(
    event: protocol::Events::UserConnected,
    context: &mut Context,
    consumer: &mut Consumer<E>,
) -> Result<(), String> {
    Err(String::from(
        "Handler for Events::UserConnected isn't implemented yet",
    ))
}
