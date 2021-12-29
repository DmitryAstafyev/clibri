#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::enum_variant_names)]

mod producer;
mod stat;
mod test;

use clibri::server;
use clibri_transport_server::{
    options::{Distributor, Listener, Options, Ports},
    server::Server,
};
use console::style;
use std::env;
use std::net::SocketAddr;
use std::ops::Range;

#[macro_export]
macro_rules! stop {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        std::process::exit(1);
    }}
}

#[derive(Debug)]
enum Port {
    Single,
    Multiple(usize),
}

const DEFAULT_CONNECTIONS: usize = 1000;
const DEFAULT_CONNECTIONS_PER_PORT: usize = 5000;

struct Configuration {
    pub port: Port,
    pub connections: usize,
    pub silent: bool,
}

impl Configuration {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        Self {
            silent: args.iter().any(|a| a.to_lowercase() == "--silent"),
            port: if let Some(arg) = args
                .iter()
                .find(|a| a.to_lowercase().contains("--multiple"))
            {
                let parts: Vec<&str> = arg.split('=').collect();
                if parts.len() == 2 {
                    match parts[1].parse::<usize>() {
                        Ok(connections) => Port::Multiple(connections),
                        Err(_) => Port::Multiple(DEFAULT_CONNECTIONS_PER_PORT),
                    }
                } else {
                    Port::Multiple(DEFAULT_CONNECTIONS_PER_PORT)
                }
            } else {
                Port::Single
            },
            connections: if let Some(arg) = args
                .iter()
                .find(|a| a.to_lowercase().contains("connections"))
            {
                let parts: Vec<&str> = arg.split('=').collect();
                if parts.len() == 2 {
                    match parts[1].parse::<usize>() {
                        Ok(connections) => connections,
                        Err(_) => DEFAULT_CONNECTIONS,
                    }
                } else {
                    DEFAULT_CONNECTIONS
                }
            } else {
                DEFAULT_CONNECTIONS
            },
        }
    }
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "- port: {:?}\n- connections: {}\n",
                self.port, self.connections
            )
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let configuration = Configuration::new();
    println!("Next configuration would be used:\n{}", configuration);
    let socket_addr = "127.0.0.1:8080"
        .parse::<SocketAddr>()
        .map_err(|e| e.to_string())?;
    let server = Server::new(Options {
        listener: if let Port::Single = configuration.port {
            Listener::Direct(socket_addr)
        } else {
            Listener::Distributor(Distributor {
                addr: String::from("127.0.0.1"),
                ports: Ports::Range(Range {
                    start: 20000,
                    end: 40000,
                }),
                distributor: socket_addr,
                connections_per_port: 2000,
            })
        },
    });
    println!("{} server is created", style("[test]").bold().dim(),);
    let context = producer::Context::new(configuration.silent);
    producer::run(server, producer::Options::new(), context)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
