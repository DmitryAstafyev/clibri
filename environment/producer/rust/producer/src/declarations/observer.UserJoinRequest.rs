use super::consumer_context::{ Context };
use super::protocol::{ StructEncode };
use super::observer::{ RequestObserverErrors };
use super::consumer_identification::EFilterMatchCondition;
use super::{ Broadcasting };
use super::Protocol;

#[derive(Debug, Clone)]
pub enum Conclusion {
    Accept(Protocol::UserJoin::Accepted),
    Deny(Protocol::UserJoin::Denied),
}

#[allow(unused_variables)]
pub trait Observer
{

    fn conclusion<UCX: 'static + Sync + Send + Clone>(
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: UCX,
        error: &dyn Fn(Protocol::UserJoin::Err) -> Result<(), RequestObserverErrors>,
    ) -> Result<Conclusion, String> {
        Err(String::from("conclusion method isn't implemented"))
    }

    fn accept<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn Context,
        ucx: UCX,
        request: Protocol::UserJoin::Request,
        broadcast: &dyn Fn(Protocol::Identification, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(Protocol::UserJoin::Err) -> Result<(), RequestObserverErrors>,
    ) -> Result<(), String> {
        Err(String::from("accept method isn't implemented"))
    }

    fn broadcast<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn Context,
        ucx: UCX,
        request: Protocol::UserJoin::Request,
        broadcast: &dyn Fn(Protocol::Identification, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(Protocol::UserJoin::Err) -> Result<(), RequestObserverErrors>,
    ) -> Result<(), String> {
        Err(String::from("broadcast method isn't implemented"))
    }

    fn deny<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn Context,
        ucx: UCX,
        request: Protocol::UserJoin::Request,
        broadcast: &dyn Fn(Protocol::Identification, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(Protocol::UserJoin::Err) -> Result<(), RequestObserverErrors>,
    ) -> Result<(), String> {
        Err(String::from("deny method isn't implemented"))
    }

    fn emit<UCX: 'static + Sync + Send + Clone>(
        &self,
        cx: &dyn Context,
        ucx: UCX,
        request: Protocol::UserJoin::Request,
        broadcast: &dyn Fn(Protocol::Identification, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        let error = |mut error: Protocol::UserJoin::Err| {
            match error.abduct() {
                Ok(buffer) => if let Err(e) = cx.send(buffer) {
                    Err(RequestObserverErrors::ResponsingError(e))
                } else {
                    Ok(())
                },
                Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
            }
        };
        match Self::conclusion(request.clone(), cx, ucx.clone(), &error) {
            Ok(conclusion) => match conclusion {
                Conclusion::Accept(mut response) => {
                    if let Err(e) = Self::accept(cx, ucx.clone(), request.clone(), broadcast, &error) {
                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                    }
                    if let Err(e) = Self::broadcast(cx, ucx.clone(), request, broadcast, &error) {
                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                    }
                    match response.abduct() {
                        Ok(buffer) => if let Err(e) = cx.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else {
                            Ok(())
                        },
                        Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                    }
                },
                Conclusion::Deny(mut response) => {
                    if let Err(e) = Self::deny(cx, ucx, request, broadcast, &error) {
                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                    }
                    match response.abduct() {
                        Ok(buffer) => if let Err(e) = cx.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else {
                            Ok(())
                        },
                        Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                    }
                },
            },
            Err(e) => Err(RequestObserverErrors::GettingConclusionError(e))
        }
    }
}

#[derive(Clone)]
pub struct ObserverRequest { }

impl ObserverRequest {
    pub fn new() -> Self {
        ObserverRequest {}
    }
}
