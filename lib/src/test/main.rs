#[allow(unused_imports)]
use super:: { protocol, buffer, msg_outgoing_builder, msg_income_extractor };
#[allow(unused_imports)]
use pingout::{PingOut, PingOutStruct};

#[path = "./main.protocol.rs"]
pub mod testprotocol;

#[path = "./main.protocol.pingout.rs"]
pub mod pingout;

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::{PingOut, PingOutStruct, msg_outgoing_builder, buffer, testprotocol };
    use testprotocol::{Messages, TestProtocol};
    use msg_outgoing_builder::Message;
    use buffer::{Processor};

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
                assert_eq!(count, 10);
            },
            Err(_) => {
                assert_eq!(true, false);
            }
        }
    }
}