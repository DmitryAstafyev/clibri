use super::{identification, Inward};
use std::collections::HashMap;
use thiserror::Error;
use tokio::{
    join, select,
    sync::{
        mpsc::{channel, unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task,
};
use uuid::Uuid;

pub type FilterCallback = dyn (Fn(&identification::Identification) -> bool) + Send;

pub type FilterCallbackBoxed = Box<FilterCallback>;

#[derive(Error, Debug)]
pub enum FilterError {
    #[error("Channel error: `{0}`")]
    Channel(String),
}

pub enum Request {
    All(oneshot::Sender<Vec<Uuid>>),
    Except(Vec<Uuid>, oneshot::Sender<Vec<Uuid>>),
    Filter(FilterCallbackBoxed, oneshot::Sender<Vec<Uuid>>),
}

pub struct Filter {
    tx_request: UnboundedSender<Request>,
}

impl Filter {
    pub fn new(tx_request: UnboundedSender<Request>) -> Self {
        Self { tx_request }
    }

    pub async fn except(&self, uuids: Vec<Uuid>) -> Result<Vec<Uuid>, FilterError> {
        let (tx_response, rx_responce): (oneshot::Sender<Vec<Uuid>>, oneshot::Receiver<Vec<Uuid>>) =
            oneshot::channel();
        self.tx_request
            .send(Request::Except(uuids, tx_response))
            .map_err(|e| FilterError::Channel(e.to_string()))?;
        rx_responce
            .await
            .map_err(|e| FilterError::Channel(e.to_string()))
    }

    pub async fn all(&self) -> Result<Vec<Uuid>, FilterError> {
        let (tx_response, rx_responce): (oneshot::Sender<Vec<Uuid>>, oneshot::Receiver<Vec<Uuid>>) =
            oneshot::channel();
        self.tx_request
            .send(Request::All(tx_response))
            .map_err(|e| FilterError::Channel(e.to_string()))?;
        rx_responce
            .await
            .map_err(|e| FilterError::Channel(e.to_string()))
    }

    // pub async fn filter(&self, cb: F) -> Result<Vec<Uuid>, FilterError>
    // where
    //     F: (Fn(&identification::Identification) -> bool) + Send,
    // {
    //     let (tx_response, rx_responce): (oneshot::Sender<Vec<Uuid>>, oneshot::Receiver<Vec<Uuid>>) =
    //         oneshot::channel();
    //     self.tx_request
    //         .send(Request::Filter(Box::new(cb), tx_response))
    //         .map_err(|e| FilterError::Channel(e.to_string()))?;
    //     rx_responce
    //         .await
    //         .map_err(|e| FilterError::Channel(e.to_string()))
    // }
}
