use super::{error::ConsumerError, protocol, Auth};
use tokio::sync::{
    mpsc::{Sender, UnboundedSender},
    oneshot,
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Debug)]
pub enum Channel {
    Send(Vec<u8>),
    Request((u32, Vec<u8>, oneshot::Sender<protocol::AvailableMessages>)),
    AcceptIncome((u32, protocol::AvailableMessages, oneshot::Sender<bool>)),
    Uuid(oneshot::Sender<Uuid>),
}

#[derive(Debug, Clone)]
pub struct Api {
    tx_client_api: UnboundedSender<Channel>,
    tx_auth: Sender<Auth>,
    shutdown: CancellationToken,
    uuid: Option<Uuid>,
}

impl Api {
    pub fn new(tx_client_api: UnboundedSender<Channel>, tx_auth: Sender<Auth>) -> Self {
        Api {
            tx_client_api,
            tx_auth,
            shutdown: CancellationToken::new(),
            uuid: None,
        }
    }

    pub async fn send(&self, buffer: &[u8]) -> Result<(), ConsumerError> {
        Ok(self
            .tx_client_api
            .send(Channel::Send(buffer.to_vec()))
            .map_err(|e| ConsumerError::APIChannel(e.to_string()))?)
    }

    pub async fn request(
        &self,
        sequence: u32,
        buffer: &[u8],
    ) -> Result<protocol::AvailableMessages, ConsumerError> {
        let (tx_response, rx_response): (
            oneshot::Sender<protocol::AvailableMessages>,
            oneshot::Receiver<protocol::AvailableMessages>,
        ) = oneshot::channel();
        self.tx_client_api
            .send(Channel::Request((sequence, buffer.to_vec(), tx_response)))
            .map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
        match rx_response.await {
            Ok(response) => Ok(response),
            Err(_) => Err(ConsumerError::GettingResponse),
        }
    }

    pub async fn accept(
        &self,
        sequence: u32,
        msg: protocol::AvailableMessages,
    ) -> Result<bool, ConsumerError> {
        let (tx_response, rx_response): (oneshot::Sender<bool>, oneshot::Receiver<bool>) =
            oneshot::channel();
        self.tx_client_api
            .send(Channel::AcceptIncome((sequence, msg, tx_response)))
            .map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
        match rx_response.await {
            Ok(response) => Ok(response),
            Err(_) => Err(ConsumerError::GettingResponse),
        }
    }

    pub fn shutdown(&self) {
        self.shutdown.cancel();
    }

    pub fn get_shutdown_token(&self) -> CancellationToken {
        self.shutdown.clone()
    }
}
