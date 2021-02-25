use super::consumer_context::{ Context };
use super::protocol::{ StructEncode };
use super::observer::{RequestObserver, RequestObserverErrors};
use super::consumer_identification::EFilterMatchCondition;
use super::{ Broadcasting };
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserSingInConclusion {
    Accept,
    Deny,
}

pub trait UserSingInObserver<
    Request: Clone,
    Response: StructEncode,
    Conclusion: Eq + Hash,
    UCX: Send + Sync,
>: RequestObserver<Request, Response, UserSingInConclusion, UCX>
{
    fn _accept(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String>;

    fn _broadcast(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String>;

    fn _deny(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String>;

    fn emit(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        match self._response(request.clone(), cx, ucx.clone()) {
            Ok((mut response, conclusion)) => match response.abduct() {
                Ok(buffer) => {
                    if let Err(e) = cx.send(buffer) {
                        Err(RequestObserverErrors::ResponsingError(e))
                    } else {
                        match conclusion {
                            UserSingInConclusion::Accept => {
                                if let Err(e) = self._accept(cx, ucx.clone(), request.clone(), broadcast) {
                                    return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                }
                                if let Err(e) = self._broadcast(cx, ucx.clone(), request, broadcast) {
                                    return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                }
                            }
                            UserSingInConclusion::Deny => {
                                if let Err(e) = self._deny(cx, ucx.clone(), request, broadcast) {
                                    return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                }
                            }
                        }
                        Ok(())
                    }
                }
                Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
            },
            Err(e) => Err(RequestObserverErrors::GettingResponseError(e)),
        }
    }
}
