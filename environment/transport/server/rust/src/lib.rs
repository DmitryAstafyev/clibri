#[macro_use]
extern crate lazy_static;

pub use tokio_tungstenite::{
    tungstenite::{
        handshake::server::{
            Request,
            Response,
            ErrorResponse,
        },
        protocol::frame::coding::CloseCode
    }
};

#[path = "./server.rs"]
pub mod server;

#[path = "./server.stat.rs"]
pub mod stat;

#[path = "./connection.handshake.rs"]
pub mod handshake;

#[path = "./connection.rs"]
pub mod connection;

#[path = "./connection.channel.rs"]
pub mod channel;

#[allow(non_upper_case_globals)]
pub mod tools {
    use fiber::logger::{ DefaultLogger };

    lazy_static! {
        pub static ref logger: DefaultLogger = DefaultLogger::new("Server".to_owned(), Some(2 /* 5 VERBOSE */));
    }

}
