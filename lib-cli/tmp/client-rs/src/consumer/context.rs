use super::implementation::protocol;
use tokio::{
    io,
    io::{AsyncBufReadExt, BufReader},
};
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
    pub async fn listen(&self) {
        while !self.shutdown.is_cancelled() {}
    }

    pub async fn get_username(&self) -> Result<String, String> {
        if let Some(input) = stdin::next_line(self.shutdown.child_token()).await? {
            Ok(input.to_owned())
        } else {
            Err(String::from("No input"))
        }
    }
}
