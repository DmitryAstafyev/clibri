use super::consumer_context::{ Context };
use super::protocol::{ StructEncode };
use super::observer::{ RequestObserver, RequestObserverErrors };
use super::consumer_identification::EFilterMatchCondition;
use super::{ Broadcasting };
use super::Protocol;
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Conclusion {
    Accept,
    Deny,
}

pub trait Observer<
    UCX: Send + Sync,
>: RequestObserver<Protocol::UserSingIn::Request, Protocol::UserSingIn::Response, Conclusion, UCX>
{
    fn accept(
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserSingIn::Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        Err(String::from("accept method isn't implemented"))
    }

    fn broadcast(
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserSingIn::Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        Err(String::from("broadcast method isn't implemented"))
    }

    fn deny(
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserSingIn::Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        Err(String::from("deny method isn't implemented"))
    }

    fn emit(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserSingIn::Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        match Self::conclusion(request.clone(), cx, ucx.clone()) {
            Ok(conclusion) => match Self::response(request.clone(), cx, ucx.clone(), conclusion.clone()) {
                Ok(mut response) => match response.abduct() {
                    Ok(buffer) => {
                        if let Err(e) = cx.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else {
                            match conclusion {
                                Conclusion::Accept => {
                                    if let Err(e) = Self::accept(cx, ucx.clone(), request.clone(), broadcast) {
                                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                    }
                                    if let Err(e) = Self::broadcast(cx, ucx.clone(), request, broadcast) {
                                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                    }
                                },
                                Conclusion::Deny => {
                                    if let Err(e) = Self::deny(cx, ucx, request, broadcast) {
                                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                    }
                                },
                            }
                            Ok(())
                        }
                    }
                    Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                },
                Err(e) => Err(RequestObserverErrors::GettingResponseError(e)),
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

impl<UCX: Send + Sync>
    RequestObserver<
        Protocol::UserSingIn::Request,
        Protocol::UserSingIn::Response,
        Conclusion,
        UCX,
    > for ObserverRequest
{ }

impl<UCX: Send + Sync> Observer<UCX> for ObserverRequest { }
