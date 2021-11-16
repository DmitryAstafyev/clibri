mod consumer;
use consumer::{connect, protocol, Consumer, Context, Options, ReconnectionStrategy};
use clibri_transport_client::{
    client,
    client::Client,
    errors::Error,
    options::{ConnectionType, Options as ClientOptions},
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), String> {
    let socket_addr = "127.0.0.1:8080"
        .parse::<SocketAddr>()
        .map_err(|e| e.to_string())?;
    let client = Client::new(ClientOptions {
        connection: ConnectionType::Direct(socket_addr),
    });
    let context = Context::new();
    let mut options = Options::defualt(protocol::Identification::SelfKey {
        uuid: None,
        id: Some(64),
        location: Some(String::from("London")),
    });
    // options.reconnection = ReconnectionStrategy::DoNotReconnect;
    let consumer = connect::<Client, Error, client::Control>(client, context, options)
        .await
        .map_err(|e| e.to_string())?;
    let shutdown = consumer
        .get()
        .await
        .map_err(|e| e.to_string())?
        .get_shutdown_token();
    shutdown.cancelled().await;
    Ok(())
}
