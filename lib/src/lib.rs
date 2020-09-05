#[path = "./server/server.rs"]
pub mod server;

#[path = "./server/server.context.rs"]
pub mod context;

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
