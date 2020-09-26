use super:: { protocol, buffer, msg_outgoing_builder, msg_income_extractor };
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
    use super::{PingOut, PingOutStruct, Uuid, msg_outgoing_builder, Processor, testprotocol };
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
                println!("{}", e);
                assert_eq!(true, false);
            }
        }
    }

    #[test]
    fn in_message_read() {
        let prot = TestProtocol {};
        let uuid = Uuid::new_v4();
        let mut buffer: Processor<Messages> = Processor::new(uuid);
        let mut buf = vec![];
        for _ in 0..10 {
            let mut ping_out_msg: PingOut = PingOut::new(PingOutStruct {
                uuid: uuid.to_string(),
            });
            match ping_out_msg.buffer() {
                Ok(mut msg_buf) => {
                    buf.append(&mut msg_buf);
                },
                Err(e) => {
                    println!("{:?}", e);
                    assert_eq!(true, false);
                }
            }
        }
        match buffer.read(&buf, prot) {
            Ok(_) => {
                let mut count = 0;
                while let Some(income) = buffer.next() {
                    match income.msg {
                        Messages::Ping(strc) => {
                            assert_eq!(strc.uuid, uuid.to_string());
                            count += 1;
                        }
                    }
                }
                assert_eq!(count, 11);
            },
            Err(_) => {
                assert_eq!(true, false);
            }
        }
    }
}