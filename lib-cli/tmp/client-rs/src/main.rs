mod consumer;
use consumer::{connect, protocol, Context, Options};
use fiber_transport_client::{
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
    let mut client = Client::new(
        ClientOptions {
            connection: ConnectionType::Direct(socket_addr),
        },
        None,
    );
    let context = Context {};
    let consumer = connect(
        client,
        context,
        Options::defualt(protocol::Identification::SelfKey {
            uuid: None,
            id: Some(64),
            location: Some(String::from("London")),
        }),
    )
    .await
    .map_err(|e| e.to_string())?;
    let shutdown = consumer.get_shutdown_token();
    shutdown.cancelled().await;
    Ok(())
    // let socket_addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    // let server = Server::new(Options {
    //     listener: Listener::Direct(socket_addr),
    // });
    // let context = producer::Context::new();
    // let manage: Manage =
    //     if let Ok(manage) = producer::run(server, producer::Options::new(), context).await {
    //         manage
    //     } else {
    //         return;
    //     };
    // manage.get_shutdown_tracker().cancelled().await;
    // println!("Chat is shutdown");
}
