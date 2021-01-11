use super::consumer_context::{Context, Encodable};
use std::cmp::Eq;
use std::hash::Hash;

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
>
{
    fn response(
        &mut self,
        request: Request,
        cx: &mut dyn Context,
    ) -> Result<(Response, Conclusion), String>;

    fn emit(
        &mut self,
        cx: &mut dyn Context,
        request: Request,
    ) -> Result<(), RequestObserverErrors> { Ok(()) }
}

pub trait ConfirmedRequestObserver<
    Request: Clone,
    Response: Encodable,
    Conclusion: Eq + Hash,
>
{

    fn conclusion(
        &mut self,
        request: Request,
        cx: &mut dyn Context,
    ) -> Result<Conclusion, String>;

    fn response(
        &mut self,
        request: Request,
        cx: &mut dyn Context,
        conclusion: Conclusion,
    ) -> Result<Response, String>;

    fn emit(
        &mut self,
        cx: &mut dyn Context,
        request: Request,
    ) -> Result<(), RequestObserverErrors> { Ok(()) }
}
