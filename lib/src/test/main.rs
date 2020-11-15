#[allow(unused_imports)]
use super:: { protocol, buffer, msg_outgoing_builder, msg_income_extractor, decode, storage, encode, package };

#[path = "./main.protocol.rs"]
pub mod testprotocol;

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::{buffer, testprotocol, encode, decode, package };
    use testprotocol::{Messages, TestProtocol, ping };
    use buffer::{Processor};
    use encode::*;

    #[test]
    fn out_message_encode() {
        let mut ping_msg: ping::Ping = ping::Ping {
            uuid: Uuid::new_v4().to_string()
        };
        match ping_msg.abduct() {
            Ok(buf) => {
                assert_eq!(buf.len(), 47);
            },
            Err(e) => {
                println!("{}", e);
                assert_eq!(true, false);
            }
        }
        match package::pack(ping_msg) {
            Ok(buf) => {
                assert_eq!(buf.len(), 67);
            }
            ,
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
            let ping_msg: ping::Ping = ping::Ping {
                uuid: uuid.to_string(),
            };
            match package::pack(ping_msg) {
                Ok(mut package_buf) => buf.append(&mut package_buf),
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