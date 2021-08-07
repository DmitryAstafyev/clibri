use super::{config, stat::Stat};
use futures::{SinkExt, StreamExt};
use std::sync::{Arc, RwLock};
use tokio::{net::TcpStream, sync::mpsc::UnboundedSender};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ClientStatus {
    Done,
    Err(String),
}

pub struct Client {
    ws: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    uuid: Uuid,
}

impl Client {
    pub async fn new(
        mut ws: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) -> Result<Self, String> {
        let ws = if let Some(ws) = ws.take() {
            ws
        } else {
            Self::connect_client().await?
        };
        Ok(Self { ws: Some(ws), uuid: Uuid::new_v4() })
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub async fn run(
        &mut self,
        status: UnboundedSender<ClientStatus>,
        stat: Arc<RwLock<Stat>>,
    ) -> Result<Uuid, String> {
        let mut ws = if let Some(ws) = self.ws.take() {
            ws
        } else {
            return Err(String::from(
                "Client isn't inited as well. No WS has been found",
            ));
        };
        // Step 1. Wakeup
        match stat.write() {
            Ok(mut stat) => stat.wakeup += 1,
            Err(err) => {
                return Err(format!("Fail write stat. Error: {}", err));
            }
        };
        // Step 2. Sending sample package to server
        let buffer: Vec<u8> = vec![0u8, 1u8, 2u8, 3u8, 4u8];
        ws.send(Message::Binary(buffer.clone()))
            .await
            .map_err(|e| e.to_string())?;
        match stat.write() {
            Ok(mut stat) => stat.write += 1,
            Err(err) => {
                return Err(format!("Fail write stat. Error: {}", err));
            }
        };
        // Step 3. Waiting and reading message from server
        if let Some(msg) = ws.next().await {
            let data = msg.unwrap().into_data();
            if data != vec![5u8, 6u8, 7u8, 8u8, 9u8] {
                return Err(String::from("Invalid data from server"));
            }
            match stat.write() {
                Ok(mut stat) => stat.read += 1,
                Err(err) => {
                    return Err(format!("Fail write stat. Error: {}", err));
                }
            };
        } else {
            return Err(String::from("Fail to get message from server"));
        }
        // Close all
        ws.close(None).await.map_err(|e| e.to_string())?;
        status.send(ClientStatus::Done).map_err(|e| e.to_string())?;
        Ok(self.uuid())
    }

    async fn connect_client() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, String> {
        match connect_async(config::CLIENT_ADDR).await {
            Ok((ws, _)) => Ok(ws),
            Err(e) => Err(format!("{}", e)),
        }
    }
}
