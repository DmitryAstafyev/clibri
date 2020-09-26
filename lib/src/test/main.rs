use super:: { protocol, buffer, session, session_context, controller, CloseFrame, Request, Response, ErrorResponse, msg_outgoing_builder, msg_income_extractor };
use buffer::{Processor};
use uuid::Uuid;
use msg_outgoing_builder::Message;
use pingout::{PingOut, PingOutStruct};

#[path = "./main.protocol.rs"]
pub mod testprotocol;

#[path = "./main.protocol.pingout.rs"]
pub mod pingout;

#[cfg(test)]
mod tests {
    use super::{PingOut, PingOutStruct, Uuid, msg_outgoing_builder, Processor, testprotocol, protocol};
    use testprotocol::{Messages, TestProtocol};
    use msg_outgoing_builder::Message;
    #[test]
    fn out_message_encode() {
        let mut ping_out_msg: PingOut = PingOut::new(PingOutStruct {
            uuid: Uuid::new_v4().to_string()
        });
        match ping_out_msg.buffer() {
            Ok(buf) => {
                assert_eq!(buf.len(), 63);
            },
            Err(e) => {
                assert_eq!(true, false);
            }
        }
    }

    fn in_message_read() {
        let prot = TestProtocol {};
        let uuid = Uuid::new_v4();
        let mut buffer: Processor<Messages> = Processor::new(uuid);
        let buf = vec![0];
        match buffer.read(&buf, prot) {
            Ok(_) => if let Some(msg) = buffer.next() {
                
            },
            Err(e) => {
                
            }
        }
    }
}

/*
fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("[LibTest] Started...");
    let listener = TcpListener::bind("127.0.0.1:8088").unwrap();
    let controller: ServerController = ServerController {};
    let mut serv: server::Server<testprotocol::Messages> = server::Server::new(controller);
    let pro = testprotocol::TestProtocol {};
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("[LibTest] Income connection!");
                let session = ClientSession {};
                match serv.add(stream, session, pro.clone()) {
                    Ok(_) => {
                        info!("[LibTest] Connection accepted!");
                    },
                    Err(e) => info!("[LibTest] Fail to add connection due error: {}", e),
                }
            },
            Err(_e) => {

            }
        }
    }
}
*/