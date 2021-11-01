use super::{protocol, Consumer, Context};

pub async fn handler(
    event: protocol::Events::UserConnected,
    context: &mut Context,
    consumer: &mut Consumer,
) -> Result<(), String> {
    Err(String::from(
        "Handler for Events::UserConnected isn't implemented yet",
    ))
}
