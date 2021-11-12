use super::{error::ConsumerError, protocol, Auth};
use fiber::client;
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
    Uuid(oneshot::Sender<Option<Uuid>>),
    Sequence(oneshot::Sender<u32>),
}

#[derive(Debug, Clone)]
pub struct Api<E: client::Error> {
    tx_client_api: UnboundedSender<Channel>,
    tx_auth: Sender<Auth<E>>,
    shutdown: CancellationToken,
    uuid: Option<Uuid>,
}

impl<E: client::Error> Api<E> {
    pub fn new(tx_client_api: UnboundedSender<Channel>, tx_auth: Sender<Auth<E>>) -> Self {
        Api {
            tx_client_api,
            tx_auth,
            shutdown: CancellationToken::new(),
            uuid: None,
        }
    }

    pub async fn sequence(&self) -> Result<u32, ConsumerError<E>> {
        let (tx_response, rx_response): (oneshot::Sender<u32>, oneshot::Receiver<u32>) =
            oneshot::channel();
        self.tx_client_api
            .send(Channel::Sequence(tx_response))
            .map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
        match rx_response.await {
            Ok(sequence) => Ok(sequence),
            Err(_) => Err(ConsumerError::GettingResponse),
        }
    }

    pub async fn send(&self, buffer: &[u8]) -> Result<(), ConsumerError<E>> {
        Ok(self
            .tx_client_api
            .send(Channel::Send(buffer.to_vec()))
            .map_err(|e| ConsumerError::APIChannel(e.to_string()))?)
    }

    pub async fn request(
        &self,
        sequence: u32,
        buffer: &[u8],
    ) -> Result<protocol::AvailableMessages, ConsumerError<E>> {
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
    ) -> Result<bool, ConsumerError<E>> {
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

    pub async fn uuid(&self) -> Result<Option<Uuid>, ConsumerError<E>> {
        let (tx_response, rx_response): (
            oneshot::Sender<Option<Uuid>>,
            oneshot::Receiver<Option<Uuid>>,
        ) = oneshot::channel();
        self.tx_client_api
            .send(Channel::Uuid(tx_response))
            .map_err(|_| ConsumerError::APIChannel(String::from("Fail to get uuid")))?;
        match rx_response.await {
            Ok(response) => Ok(response),
            Err(_) => Err(ConsumerError::GettingResponse),
        }
    }

    pub async fn uuid_as_string(&self) -> Result<Option<String>, ConsumerError<E>> {
        if let Some(uuid) = self.uuid().await? {
            Ok(Some(uuid.to_string()))
        } else {
            Ok(None)
        }
    }

    pub fn shutdown(&self) {
        self.shutdown.cancel();
    }

    pub fn get_shutdown_token(&self) -> CancellationToken {
        self.shutdown.clone()
    }
}
