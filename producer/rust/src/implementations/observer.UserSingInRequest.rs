use super::consumer_context::{Context, Encodable};
use super::observer::{ RequestObserver };
use super::DeclUserSingInRequest::{ UserSingInObserver, UserSingInConclusion };
use super::consumer_identification::EFilterMatchCondition;
use super::{ Broadcasting, UserCustomContext };
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct UserSingInRequest {
    pub login: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct UserSingInResponse {
    error: Option<String>,
}

impl Encodable for UserSingInResponse {
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

type TEventHandler = &'static (dyn (Fn(
    UserSingInRequest,
    &dyn Context,
    Arc<RwLock<UserCustomContext>>,
    &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
) -> Result<(), String>)
              + Send
              + Sync);
type TResponseHandler = &'static (dyn (Fn(
    UserSingInRequest,
    &dyn Context,
    Arc<RwLock<UserCustomContext>>,
) -> Result<(UserSingInResponse, UserSingInConclusion), String>)
                + Send
                + Sync);

#[derive(Clone)]
pub struct ObserverRequest {
    response: Option<TResponseHandler>,
    accept: Option<TEventHandler>,
    broadcast: Option<TEventHandler>,
    deny: Option<TEventHandler>,
}

impl ObserverRequest {

    pub fn new() -> Self {
        ObserverRequest {
            response: None,
            accept: None,
            broadcast: None,
            deny: None,
        }
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

impl RequestObserver<UserSingInRequest, UserSingInResponse, UserSingInConclusion, UserCustomContext> for ObserverRequest {

    fn _response(
        &self,
        request: UserSingInRequest,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
    ) -> Result<(UserSingInResponse, UserSingInConclusion), String> {
        if let Some(handler) = self.response {
            handler(request, cx, ucx)
        } else {
            panic!("UserSingInRequest: no handler for [response]")
        }
    }
}

impl UserSingInObserver<UserSingInRequest, UserSingInResponse, UserSingInConclusion, UserCustomContext> for ObserverRequest {

    fn _accept(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        request: UserSingInRequest,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.accept {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("UserSingInRequest: no handler for [accept]")
        }
    }

    fn _broadcast(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        request: UserSingInRequest,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.broadcast {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("UserSingInRequest: no handler for [broadcast]")
        }
    }

    fn _deny(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        request: UserSingInRequest,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(handler) = self.deny {
            handler(request, cx, ucx, broadcast)
        } else {
            panic!("UserSingInRequest: no handler for [deny]")
        }
    }
}