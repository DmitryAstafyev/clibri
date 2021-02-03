use super::consumer_context::{Context, Encodable};
use super::consumer_identification::EFilterMatchCondition;
use super::observer::ConfirmedRequestObserver;
use super::DeclUserJoinRequest::{UserJoinConclusion, UserJoinObserver};
use super::{Broadcasting, UserCustomContext};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct UserJoinRequest {
    pub login: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct UserJoinResponse {
    pub error: Option<String>,
}

impl Encodable for UserJoinResponse {
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

type TConclusionHandler = &'static (dyn (Fn(
    UserJoinRequest,
    &dyn Context,
    Arc<RwLock<UserCustomContext>>,
) -> Result<UserJoinConclusion, String>)
              + Send
              + Sync);
type TResponseHandler = &'static (dyn (Fn(
    UserJoinRequest,
    &dyn Context,
    Arc<RwLock<UserCustomContext>>,
    UserJoinConclusion,
) -> Result<UserJoinResponse, String>)
              + Send
              + Sync);
type TEventHandler = &'static (dyn (Fn(
    UserJoinRequest,
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
        UserJoinRequest,
        UserJoinResponse,
        UserJoinConclusion,
        UserCustomContext,
    > for ObserverRequest
{
    fn _conclusion(
        &mut self,
        request: UserJoinRequest,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
    ) -> Result<UserJoinConclusion, String> {
        if let Some(handler) = self.conclusion {
            handler(request, cx, ucx)
        } else {
            panic!("UserJoinRequest: no handler for [conclution]")
        }
    }

    fn _response(
        &mut self,
        request: UserJoinRequest,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        conclusion: UserJoinConclusion,
    ) -> Result<UserJoinResponse, String> {
        if let Some(handler) = self.response {
            handler(request, cx, ucx, conclusion)
        } else {
            panic!("UserJoinRequest: no handler for [conclution]")
        }
    }
}

impl UserJoinObserver<UserJoinRequest, UserJoinResponse, UserJoinConclusion, UserCustomContext>
    for ObserverRequest
{
    fn _accept(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        request: UserJoinRequest,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.accept {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("UserJoinRequest: no handler for [accept]")
        }
    }

    fn _broadcast(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        request: UserJoinRequest,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.broadcast {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("UserJoinRequest: no handler for [broadcast]")
        }
    }

    fn _deny(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        request: UserJoinRequest,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.deny {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("UserJoinRequest: no handler for [deny]")
        }
    }
}
