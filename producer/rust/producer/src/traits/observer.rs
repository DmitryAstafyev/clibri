use super::consumer_context::{ Context };
use super::protocol::{ StructEncode };
use std::cmp::Eq;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub enum RequestObserverErrors {
    ResponsingError(String),
    GettingResponseError(String),
    EncodingResponseError(String),
    BeforeResponseActionFail(String),
    ErrorOnEventsEmit(String),
    GettingConclusionError(String),
}

pub trait RequestObserver<
    Request: Clone,
    Response: StructEncode,
    Conclusion: Eq + Hash,
    UCX: Send + Sync,
>
{

    fn conclusion(
        request: Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<Conclusion, String> {
        Err(String::from("conclusion method isn't implemented"))
    }

    fn response(
        request: Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        conclusion: Conclusion,
    ) -> Result<Response, String> {
        Err(String::from("response method isn't implemented"))
    }

}

#[derive(Debug)]
pub enum EventObserverErrors {
    ResponsingError(String),
    GettingResponseError(String),
    EncodingResponseError(String),
    BeforeResponseActionFail(String),
    ErrorOnEventsEmit(String),
    GettingConclusionError(String),
}
