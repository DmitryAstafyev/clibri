use super::implementation::{controller, protocol, Consumer};
use fiber_transport_client::errors::Error as ClientError;
use thiserror::Error;
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
}

impl Context {
    pub fn new() -> Self {
        Context {
            shutdown: CancellationToken::new(),
        }
    }
    pub async fn listen(&self, username: String, mut consumer: Consumer<ClientError>) {
        let shutdown = self.shutdown.clone();
        spawn(async move {
            if let Err(err) = select! {
                res = async {
                    loop {
                        if let Some(input) = stdin::next_line(shutdown.child_token()).await? {
                            match consumer
                                .request_message(protocol::Message::Request {
                                    user: username.clone(),
                                    message: input.to_owned(),
                                })
                                .await
                            {
                                Ok(response) => match response {
                                    controller::RequestMessageResponse::Accepted(_) => {
                                        println!("{}: {}", username, input);
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
                    }
                    Ok::<(), String>(())
                } => res,
                _ = shutdown.cancelled() => {
                    Ok(())
                },
            } {
                eprintln!("{}", err);
            }
        });
    }

    pub async fn get_username(&self) -> Result<String, String> {
        if let Some(input) = stdin::next_line(self.shutdown.child_token()).await? {
            Ok(input.to_owned())
        } else {
            Err(String::from("No input"))
        }
    }

    pub fn shutdown(&self) {
        println!("chat is shutdown for you");
        self.shutdown.cancel()
    }
}
