mod producer;

use fiber_transport_server::{
    options::{Listener, Options},
    server::Server,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let socket_addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let server = Server::new(Options {
        listener: Listener::Direct(socket_addr),
    });
    let context = producer::Context::new();
    producer::run(server, context).await;
    println!("Hello, World!");
}
