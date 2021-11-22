mod connection;
mod consumer;
mod test;

use connection::Connection;

#[tokio::main]
async fn main() -> Result<(), String> {
    let conn = Connection::new("127.0.0.1:8080");
    if let Err(err) = conn.connect().await {
        panic!("{}", err);
    }
    // let context = Context::new();
    // let mut options = Options::defualt(protocol::Identification::SelfKey {
    //     uuid: None,
    //     id: Some(64),
    //     location: Some(String::from("London")),
    // });
    // // options.reconnection = ReconnectionStrategy::DoNotReconnect;
    // let consumer = connect::<Client, Error, client::Control>(client, context, options)
    //     .await
    //     .map_err(|e| e.to_string())?;
    // let shutdown = consumer
    //     .get()
    //     .await
    //     .map_err(|e| e.to_string())?
    //     .get_shutdown_token();
    // shutdown.cancelled().await;
    Ok(())
}
