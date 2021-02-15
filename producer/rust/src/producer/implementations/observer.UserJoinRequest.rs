use super::consumer_context::{ Context };
use super::consumer_identification::EFilterMatchCondition;
use super::observer::ConfirmedRequestObserver;
use super::DeclUserJoinRequest::{UserJoinConclusion, UserJoinObserver};
use super::{Broadcasting};
use super::Protocol;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

type TConclusionHandler<UCX> = &'static (dyn (Fn(
    Protocol::UserJoin::Request,
    &dyn Context,
    Arc<RwLock<UCX>>,
) -> Result<UserJoinConclusion, String>)
              + Send
              + Sync);
type TResponseHandler<UCX> = &'static (dyn (Fn(
    Protocol::UserJoin::Request,
    &dyn Context,
    Arc<RwLock<UCX>>,
    UserJoinConclusion,
) -> Result<Protocol::UserJoin::Response, String>)
              + Send
              + Sync);
type TEventHandler<UCX> = &'static (dyn (Fn(
    Protocol::UserJoin::Request,
    &dyn Context,
    Arc<RwLock<UCX>>,
    &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
) -> Result<(), String>)
              + Send
              + Sync);

#[derive(Clone)]
pub struct ObserverRequest<UCX: 'static + Send + Sync> {
    conclusion: Option<TConclusionHandler<UCX>>,
    response: Option<TResponseHandler<UCX>>,
    accept: Option<TEventHandler<UCX>>,
    broadcast: Option<TEventHandler<UCX>>,
    deny: Option<TEventHandler<UCX>>,
}

impl<UCX: Send + Sync> ObserverRequest<UCX> {
    pub fn new() -> Self {
        ObserverRequest {
            conclusion: None,
            response: None,
            accept: None,
            broadcast: None,
            deny: None,
        }
    }

    pub fn conclusion(&mut self, handler: TConclusionHandler<UCX>) {
        self.conclusion = Some(handler);
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

impl<UCX: Send + Sync>
    ConfirmedRequestObserver<
        Protocol::UserJoin::Request,
        Protocol::UserJoin::Response,
        UserJoinConclusion,
        UCX,
    > for ObserverRequest<UCX>
{
    fn _conclusion(
        &self,
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<UserJoinConclusion, String> {
        if let Some(handler) = self.conclusion {
            handler(request, cx, ucx)
        } else {
            panic!("Protocol::UserJoin::Request: no handler for [conclution]")
        }
    }

    fn _response(
        &self,
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        conclusion: UserJoinConclusion,
    ) -> Result<Protocol::UserJoin::Response, String> {
        if let Some(handler) = self.response {
            handler(request, cx, ucx, conclusion)
        } else {
            panic!("Protocol::UserJoin::Request: no handler for [conclution]")
        }
    }
}

impl<UCX: Send + Sync> UserJoinObserver<Protocol::UserJoin::Request, Protocol::UserJoin::Response, UserJoinConclusion, UCX>
    for ObserverRequest<UCX>
{
    fn _accept(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserJoin::Request,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.accept {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("Protocol::UserJoin::Request: no handler for [accept]")
        }
    }

    fn _broadcast(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserJoin::Request,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.broadcast {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("Protocol::UserJoin::Request: no handler for [broadcast]")
        }
    }

    fn _deny(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserJoin::Request,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.deny {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("Protocol::UserJoin::Request: no handler for [deny]")
        }
    }
}
