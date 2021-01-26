use super::consumer_context::{Context, Encodable};
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
    Response: Encodable,
    Conclusion: Eq + Hash,
    UCX: Send + Sync,
>
{
    fn response(
        &mut self,
        request: Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<(Response, Conclusion), String>;

    fn emit(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Request,
    ) -> Result<(), RequestObserverErrors> { Ok(()) }
}

pub trait ConfirmedRequestObserver<
    Request: Clone,
    Response: Encodable,
    Conclusion: Eq + Hash,
    UCX: Send + Sync,
>
{

    fn conclusion(
        &mut self,
        request: Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<Conclusion, String>;

    fn response(
        &mut self,
        request: Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        conclusion: Conclusion,
    ) -> Result<Response, String>;

    fn emit(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Request,
    ) -> Result<(), RequestObserverErrors> { Ok(()) }
}
