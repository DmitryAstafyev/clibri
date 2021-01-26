use super::consumer_context::{Context, Encodable};
use super::observer::{ConfirmedRequestObserver, RequestObserverErrors};
use super::consumer_identification::EFilterMatchCondition;
use super::{ Broadcasting };
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserJoinConclusion {
    Accept,
    Deny,
}

pub trait UserJoinObserver<
    Request: Clone,
    Response: Encodable,
    Conclusion: Eq + Hash,
    UCX: Send + Sync,
>: ConfirmedRequestObserver<Request, Response, UserJoinConclusion, UCX>
{
    fn accept(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Request,
    ) -> Result<(), String>;

    fn broadcast(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String>;

    fn deny(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Request,
    ) -> Result<(), String>;

    fn emit(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        match self.conclusion(request.clone(), cx, ucx.clone()) {
            Ok(conclusion) => match self.response(request.clone(), cx, ucx.clone(), conclusion.clone()) {
                Ok(mut response) => match response.abduct() {
                    Ok(buffer) => {
                        if let Err(e) = cx.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else {
                            match conclusion {
                                UserJoinConclusion::Accept => {
                                    if let Err(e) = self.accept(cx, ucx.clone(), request.clone()) {
                                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                    }
                                    if let Err(e) = self.broadcast(cx, ucx.clone(), request, broadcast) {
                                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                    }
                                },
                                UserJoinConclusion::Deny => {
                                    if let Err(e) = self.deny(cx, ucx, request) {
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
