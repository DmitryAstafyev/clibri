mod producer;
mod stat;
mod test;

use clibri_transport_server::{
    options::{Listener, Options},
    server::Server,
};
use producer::Manage;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let socket_addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let server = Server::new(Options {
        listener: Listener::Direct(socket_addr),
    });
    let context = producer::Context::new();
    let manage: Manage =
        if let Ok(manage) = producer::run(server, producer::Options::new(), context).await {
            manage
        } else {
            return;
        };
    manage.get_shutdown_tracker().cancelled().await;
}
