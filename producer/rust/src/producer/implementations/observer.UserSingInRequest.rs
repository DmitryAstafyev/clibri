use super::consumer_context::{ Context };
use super::observer::{ RequestObserver };
use super::DeclUserSingInRequest::{ UserSingInObserver, UserSingInConclusion };
use super::consumer_identification::EFilterMatchCondition;
use super::{ Broadcasting };
use super::Protocol;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

type TEventHandler<UCX> = &'static (dyn (Fn(
    Protocol::UserSingIn::Request,
    &dyn Context,
    Arc<RwLock<UCX>>,
    &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
) -> Result<(), String>)
              + Send
              + Sync);
type TResponseHandler<UCX> = &'static (dyn (Fn(
    Protocol::UserSingIn::Request,
    &dyn Context,
    Arc<RwLock<UCX>>,
) -> Result<(Protocol::UserSingIn::Response, UserSingInConclusion), String>)
                + Send
                + Sync);

#[derive(Clone)]
pub struct ObserverRequest<UCX: 'static + Send + Sync> {
    response: Option<TResponseHandler<UCX>>,
    accept: Option<TEventHandler<UCX>>,
    broadcast: Option<TEventHandler<UCX>>,
    deny: Option<TEventHandler<UCX>>,
}

impl<UCX: Send + Sync> ObserverRequest<UCX> {

    pub fn new() -> Self {
        ObserverRequest {
            response: None,
            accept: None,
            broadcast: None,
            deny: None,
        }
    }

    pub fn response(&mut self, handler: TResponseHandler<UCX>) {
        self.response = Some(handler);
    }

    pub fn accept(&mut self, handler: TEventHandler<UCX>) {
        self.accept = Some(handler);
    }

    pub fn broadcast(&mut self, handler: TEventHandler<UCX>) {
        self.broadcast = Some(handler);
    }

    pub fn deny(&mut self, handler: TEventHandler<UCX>) {
        self.deny = Some(handler);
    }

}

impl<UCX: Send + Sync> RequestObserver<Protocol::UserSingIn::Request, Protocol::UserSingIn::Response, UserSingInConclusion, UCX> for ObserverRequest<UCX> {

    fn _response(
        &self,
        request: Protocol::UserSingIn::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<(Protocol::UserSingIn::Response, UserSingInConclusion), String> {
        if let Some(handler) = self.response {
            handler(request, cx, ucx)
        } else {
            panic!("Protocol::UserSingIn::Request: no handler for [response]")
        }
    }
}

impl<UCX: Send + Sync> UserSingInObserver<Protocol::UserSingIn::Request, Protocol::UserSingIn::Response, UserSingInConclusion, UCX> for ObserverRequest<UCX> {

    fn _accept(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserSingIn::Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.accept {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("Protocol::UserSingIn::Request: no handler for [accept]")
        }
    }

    fn _broadcast(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserSingIn::Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.broadcast {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("Protocol::UserSingIn::Request: no handler for [broadcast]")
        }
    }

    fn _deny(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserSingIn::Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.deny {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("Protocol::UserSingIn::Request: no handler for [deny]")
        }
    }
}