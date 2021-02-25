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
    fn _response(
        &self,
        request: Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<(Response, Conclusion), String>;

}

pub trait ConfirmedRequestObserver<
    Request: Clone,
    Response: StructEncode,
    Conclusion: Eq + Hash,
    UCX: Send + Sync,
>
{

    fn _conclusion(
        &self,
        request: Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<Conclusion, String>;

    fn _response(
        &self,
        request: Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        conclusion: Conclusion,
    ) -> Result<Response, String>;

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
