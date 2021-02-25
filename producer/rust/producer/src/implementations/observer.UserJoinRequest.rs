use super::consumer_context::Context;
use super::consumer_identification::EFilterMatchCondition;
use super::observer::ConfirmedRequestObserver;
use super::Broadcasting;
use super::DeclUserJoinRequest::{ UserJoinConclusion, UserJoinObserver };
use super::Protocol;
use super::Context as UCX;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait ObserverRequestInterface {
    fn conclusion(
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<UserJoinConclusion, String> {
        Err(String::from("conclusion method isn't implemented"))
    }

    fn response(
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        conclusion: UserJoinConclusion,
    ) -> Result<Protocol::UserJoin::Response, String> {
        Err(String::from("response method isn't implemented"))
    }

    fn accept(
        request: Protocol::UserJoin::Request,
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
        request: Protocol::UserJoin::Request,
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
        request: Protocol::UserJoin::Request,
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

impl
    ConfirmedRequestObserver<
        Protocol::UserJoin::Request,
        Protocol::UserJoin::Response,
        UserJoinConclusion,
        UCX,
    > for ObserverRequest
{
    fn _conclusion(
        &self,
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<UserJoinConclusion, String> {
        ObserverRequest::conclusion(request, cx, ucx)
    }

    fn _response(
        &self,
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        conclusion: UserJoinConclusion,
    ) -> Result<Protocol::UserJoin::Response, String> {
        ObserverRequest::response(request, cx, ucx, conclusion)
    }
}

impl
    UserJoinObserver<
        Protocol::UserJoin::Request,
        Protocol::UserJoin::Response,
        UserJoinConclusion,
        UCX,
    > for ObserverRequest
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
        ObserverRequest::accept(request, cx, ucx, broadcast)
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
        ObserverRequest::broadcast(request, cx, ucx, broadcast)
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
        ObserverRequest::deny(request, cx, ucx, broadcast)
    }
}
