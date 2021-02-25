use super::consumer_context::Context;
use super::consumer_identification::EFilterMatchCondition;
use super::observer::RequestObserver;
use super::Broadcasting;
use super::DeclUserSingInRequest::{UserSingInConclusion, UserSingInObserver};
use super::Protocol;
use super::Context as UCX;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait ObserverRequestInterface {
    fn response(
        request: Protocol::UserSingIn::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<(Protocol::UserSingIn::Response, UserSingInConclusion), String> {
        Err(String::from("response method isn't implemented"))
    }

    fn accept(
        request: Protocol::UserSingIn::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        broadcasting: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        Err(String::from("accept method isn't implemented"))
    }

    fn broadcast(
        request: Protocol::UserSingIn::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        broadcasting: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        Err(String::from("broadcast method isn't implemented"))
    }

    fn deny(
        request: Protocol::UserSingIn::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        broadcasting: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        Err(String::from("deny method isn't implemented"))
    }
}

#[derive(Clone)]
pub struct ObserverRequest {}

impl ObserverRequest {
    pub fn new() -> Self {
        ObserverRequest {}
    }
}

impl ObserverRequestInterface for ObserverRequest {}

impl
    RequestObserver<
        Protocol::UserSingIn::Request,
        Protocol::UserSingIn::Response,
        UserSingInConclusion,
        UCX,
    > for ObserverRequest
{
    fn _response(
        &self,
        request: Protocol::UserSingIn::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<(Protocol::UserSingIn::Response, UserSingInConclusion), String> {
        ObserverRequest::response(request, cx, ucx)
    }
}

impl
    UserSingInObserver<
        Protocol::UserSingIn::Request,
        Protocol::UserSingIn::Response,
        UserSingInConclusion,
        UCX,
    > for ObserverRequest
{
    fn _accept(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserSingIn::Request,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        ObserverRequest::accept(request, cx, ucx, broadcast)
    }

    fn _broadcast(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserSingIn::Request,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        ObserverRequest::broadcast(request, cx, ucx, broadcast)
    }

    fn _deny(
        &self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: Protocol::UserSingIn::Request,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        ObserverRequest::deny(request, cx, ucx, broadcast)
    }
}
