use super::{protocol, Consumer, Context};

pub async fn handler<E: std::error::Error>(
    event: protocol::Events::Message,
    context: &mut Context,
    consumer: &mut Consumer<E>,
) -> Result<(), String> {
    Err(String::from(
        "Handler for Events::Message isn't implemented yet",
    ))
}
