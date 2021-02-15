use super::consumer_context::{ Context };
use super::consumer_identification::EFilterMatchCondition;
use super::observer::ConfirmedRequestObserver;
use super::DeclUserJoinRequest::{UserJoinConclusion, UserJoinObserver};
use super::{Broadcasting, UserCustomContext};
use super::Protocol;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

type TConclusionHandler = &'static (dyn (Fn(
    Protocol::UserJoin::Request,
    &dyn Context,
    Arc<RwLock<UserCustomContext>>,
) -> Result<UserJoinConclusion, String>)
              + Send
              + Sync);
type TResponseHandler = &'static (dyn (Fn(
    Protocol::UserJoin::Request,
    &dyn Context,
    Arc<RwLock<UserCustomContext>>,
    UserJoinConclusion,
) -> Result<Protocol::UserJoin::Response, String>)
              + Send
              + Sync);
type TEventHandler = &'static (dyn (Fn(
    Protocol::UserJoin::Request,
    &dyn Context,
    Arc<RwLock<UserCustomContext>>,
    &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
) -> Result<(), String>)
              + Send
              + Sync);

#[derive(Clone)]
pub struct ObserverRequest {
    conclusion: Option<TConclusionHandler>,
    response: Option<TResponseHandler>,
    accept: Option<TEventHandler>,
    broadcast: Option<TEventHandler>,
    deny: Option<TEventHandler>,
}

impl ObserverRequest {
    pub fn new() -> Self {
        ObserverRequest {
            conclusion: None,
            response: None,
            accept: None,
            broadcast: None,
            deny: None,
        }
    }

    pub fn conclusion(&mut self, handler: TConclusionHandler) {
        self.conclusion = Some(handler);
    }

    pub fn response(&mut self, handler: TResponseHandler) {
        self.response = Some(handler);
    }

    pub fn accept(&mut self, handler: TEventHandler) {
        self.accept = Some(handler);
    }

    pub fn broadcast(&mut self, handler: TEventHandler) {
        self.broadcast = Some(handler);
    }

    pub fn deny(&mut self, handler: TEventHandler) {
        self.deny = Some(handler);
    }
}

impl
    ConfirmedRequestObserver<
        Protocol::UserJoin::Request,
        Protocol::UserJoin::Response,
        UserJoinConclusion,
        UserCustomContext,
    > for ObserverRequest
{
    fn _conclusion(
        &self,
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
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
        ucx: Arc<RwLock<UserCustomContext>>,
        conclusion: UserJoinConclusion,
    ) -> Result<Protocol::UserJoin::Response, String> {
        if let Some(handler) = self.response {
            handler(request, cx, ucx, conclusion)
        } else {
            panic!("Protocol::UserJoin::Request: no handler for [conclution]")
        }
    }
}

impl UserJoinObserver<Protocol::UserJoin::Request, Protocol::UserJoin::Response, UserJoinConclusion, UserCustomContext>
    for ObserverRequest
{
    fn _accept(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
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
        ucx: Arc<RwLock<UserCustomContext>>,
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
        ucx: Arc<RwLock<UserCustomContext>>,
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
