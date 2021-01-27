use super::consumer_context::{Context, Encodable};
use super::observer::{ ConfirmedRequestObserver };
use super::DeclUserJoinRequest::{ UserJoinObserver, UserJoinConclusion };
use super::consumer_identification::EFilterMatchCondition;
use super::{ Broadcasting, UserCustomContext };
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct UserJoinRequest {
    pub login: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct UserJoinResponse {
    error: Option<String>,
}

impl Encodable for UserJoinResponse {
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

impl ConfirmedRequestObserver<UserJoinRequest, UserJoinResponse, UserJoinConclusion, UserCustomContext> for ObserverRequest {

    fn conclusion(
        &mut self,
        request: UserJoinRequest,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
    ) -> Result<UserJoinConclusion, String> {
        Ok(UserJoinConclusion::Accept)
    }

    fn response(
        &mut self,
        request: UserJoinRequest,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        conclusion: UserJoinConclusion,
    ) -> Result<UserJoinResponse, String> {
        Ok(UserJoinResponse { error: None })
    }
}

impl UserJoinObserver<UserJoinRequest, UserJoinResponse, UserJoinConclusion, UserCustomContext> for ObserverRequest {

    fn accept(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        request: UserJoinRequest,
    ) -> Result<(), String> {
        Ok(())
    }

    fn broadcast(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        request: UserJoinRequest,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        Ok(())
    }

    fn deny(
        &mut self,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        request: UserJoinRequest,
    ) -> Result<(), String> {
        Ok(())
    }
}
