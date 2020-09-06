pub use tungstenite::handshake::server::{ Request, Response, ErrorResponse };
pub use tungstenite::protocol::{ CloseFrame };

#[path = "./server/server.rs"]
pub mod server;

#[path = "./controller/controller.rs"]
pub mod controller;

#[path = "./session/session.context.rs"]
pub mod session_context;

#[path = "./connection/connection.rs"]
pub mod connection;

#[path = "./connection/connection.channel.rs"]
pub mod connection_channel;

#[path = "./connection/connection.buffer.rs"]
pub mod buffer;

#[path = "./session/session.rs"]
pub mod session;

#[path = "./protocol/protocol.rs"]
pub mod protocol;

#[path = "./connection/connection.message.income.extractor.rs"]
pub mod msg_income_extractor;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
