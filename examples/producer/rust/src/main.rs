mod producer;
use clibri_transport_server::{
    options::{Listener, Options},
    server::Server,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), String> {
    let socket_addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let server = Server::new(Options {
        listener: Listener::Direct(socket_addr),
    });
    let context = producer::Context::new();
    producer::run(server, producer::Options::new(), context)
        .await
        .map_err(|e| e.to_string())?;
    println!("Chat is shutdown");
    Ok(())
}
