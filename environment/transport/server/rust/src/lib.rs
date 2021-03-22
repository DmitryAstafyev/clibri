#[macro_use]
extern crate lazy_static;

pub use tungstenite::handshake::server::{ Request, Response, ErrorResponse };
pub use tungstenite::protocol::{ CloseFrame };

#[path = "./server/server.rs"]
pub mod server;

#[path = "./connection/connection.rs"]
pub mod connection;

#[path = "./connection/connection.channel.rs"]
pub mod connection_channel;

#[path = "./connection/connection.context.rs"]
pub mod connection_context;

pub mod tools {
    use fiber::logger::{ DefaultLogger };

    lazy_static! {
        pub static ref logger: DefaultLogger = DefaultLogger::new("Server".to_owned(), None);
    }

}

