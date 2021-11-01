use super::{protocol, Consumer, Context};

pub async fn handler(
    event: protocol::Events::Message,
    context: &mut Context,
    consumer: &mut Consumer,
) -> Result<(), String> {
    Err(String::from(
        "Handler for Events::Message isn't implemented yet",
    ))
}
