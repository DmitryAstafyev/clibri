use super::implementation::{controller, protocol, Consumer};
use chrono::{DateTime, Local, TimeZone};
use fiber::client;
use tokio::{select, task::spawn};
use tokio_util::sync::CancellationToken;

mod stdin {
    use std::str::from_utf8;
    use tokio::{
        io,
        io::{AsyncBufReadExt, BufReader},
        select,
    };
    use tokio_util::sync::CancellationToken;

    pub async fn next_line(cancel: CancellationToken) -> Result<Option<String>, String> {
        let mut reader = BufReader::new(io::stdin());
        let mut buffer = Vec::new();
        select! {
            _ = reader.read_until(b'\n', &mut buffer) => {
                Ok(Some(from_utf8(&buffer).map_err(|e| e.to_string())?.to_string()))
            },
            _ = cancel.cancelled() => {
                Ok(None)
            }
        }
    }
}

pub struct Context {
    shutdown: CancellationToken,
    username: String,
}

impl Context {
    pub fn new() -> Self {
        Context {
            shutdown: CancellationToken::new(),
            username: String::new(),
        }
    }
    pub fn reinit(&mut self) {
        self.shutdown = CancellationToken::new();
    }
    pub async fn listen<E: client::Error>(&self, username: String, mut consumer: Consumer<E>) {
        let shutdown = self.shutdown.clone();
        spawn(async move {
            println!("Start chat");
            println!("{}[2J", 27 as char);
            println!("Type your message and press ENTER to post");
            if let Err(err) = select! {
                res = async {
                    while let Some(input) = stdin::next_line(shutdown.child_token()).await? {
                        let msg = input.trim().to_owned();
                        println!("{}[2A", 27 as char);
                        println!("{}[2K", 27 as char);
                        match consumer
                            .request_message(protocol::Message::Request {
                                user: username.clone(),
                                message: msg.clone(),
                            })
                            .await
                        {
                            Ok(response) => match response {
                                controller::RequestMessageResponse::Accepted(_) => {
                                    println!("[{}] {}: {}", Local::now().format("%H:%M:%S"), username, msg);
                                }
                                controller::RequestMessageResponse::Denied(_) => {
                                    eprintln!("[ERROR]: message is rejected");
                                }
                                controller::RequestMessageResponse::Err(msg) => {
                                    eprintln!("[ERROR]: message is rejected: {}", msg.error);
                                }
                            },
                            Err(err) => {
                                eprintln!("{}", err);
                                break;
                            }
                        }
                    }
                    Ok::<(), String>(())
                } => res,
                _ = shutdown.cancelled() => {
                    Ok(())
                },
            } {
                eprintln!("{}", err);
            }
            println!("Stop chat");
        });
    }

    pub async fn get_username(&mut self) -> Result<String, String> {
        if self.username.is_empty() {
            if let Some(input) = stdin::next_line(self.shutdown.child_token()).await? {
                self.username = input.to_owned();
            } else {
                return Err(String::from("No input"));
            }
        }
        Ok(self.username.clone())
    }

    pub fn shutdown(&self) {
        println!("chat is shutdown for you");
        self.shutdown.cancel()
    }

    pub fn get_localtime(&self, timestamp: u64) -> String {
        let dt: DateTime<Local> = Local.timestamp(timestamp as i64, 0);
        dt.format("%H:%M:%S").to_string()
    }
}
