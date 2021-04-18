#[macro_use]
extern crate lazy_static;

pub use async_tungstenite::{
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
        pub static ref logger: DefaultLogger = DefaultLogger::new("Server".to_owned(), None);
    }

}

