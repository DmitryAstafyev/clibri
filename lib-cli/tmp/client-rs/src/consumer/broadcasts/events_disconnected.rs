use super::{protocol, Consumer, Context};

pub async fn handler(
    event: protocol::Events::UserDisconnected,
    context: &mut Context,
    consumer: &mut Consumer,
) -> Result<(), String> {
    Err(String::from(
        "Handler for Events::UserDisconnected isn't implemented yet",
    ))
}
