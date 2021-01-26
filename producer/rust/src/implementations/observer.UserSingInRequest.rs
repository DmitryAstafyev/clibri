use super::consumer_context::{Context, Encodable};
use super::observer::{ RequestObserver };
use super::DeclUserSingInRequest::{ UserSingInObserver, UserSingInConclusion };
use super::consumer_identification::EFilterMatchCondition;
use super::{ Broadcasting };
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

#[derive(Debug, Clone)]
pub struct ObserverRequest {

}

impl ObserverRequest {

    pub fn new() -> Self {
        ObserverRequest {}
    }

}

impl<UCX> RequestObserver<UserSingInRequest, UserSingInResponse, UserSingInConclusion, UCX> for ObserverRequest where UCX: Send + Sync {

    fn response(
        &mut self,
        request: UserSingInRequest,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
    ) -> Result<(UserSingInResponse, UserSingInConclusion), String> {
        Ok((UserSingInResponse { error: None }, UserSingInConclusion::Accept))
    }
}

impl<UCX> UserSingInObserver<UserSingInRequest, UserSingInResponse, UserSingInConclusion, UCX> for ObserverRequest where UCX: Send + Sync {

    fn accept(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: UserSingInRequest,
    ) -> Result<(), String> {
        Ok(())
    }

    fn broadcast(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: UserSingInRequest,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        Ok(())
    }

    fn deny(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UCX>>,
        request: UserSingInRequest,
    ) -> Result<(), String> {
        Ok(())
    }
}