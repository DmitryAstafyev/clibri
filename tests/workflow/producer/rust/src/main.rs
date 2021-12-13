#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::enum_variant_names)]

mod producer;
mod stat;
mod test;

use clibri::server;
use clibri_transport_server::{
    options::{Listener, Options},
    server::Server,
};
use console::style;
use std::net::SocketAddr;

#[macro_export]
macro_rules! stop {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        std::process::exit(1);
    }}
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let socket_addr = "127.0.0.1:8080"
        .parse::<SocketAddr>()
        .map_err(|e| e.to_string())?;
    let server = Server::new(Options {
        listener: Listener::Direct(socket_addr),
    });
    println!("{} server is created", style("[test]").bold().dim(),);
    let context = producer::Context::new();
    producer::run(server, producer::Options::new(), context)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
