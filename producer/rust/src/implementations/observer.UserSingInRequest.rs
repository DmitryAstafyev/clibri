use super::consumer_context::{Context, Encodable};
use super::observer::{ RequestObserver };
use super::DeclUserSingInRequest::{ UserSingInObserver, UserSingInConclusion };

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

impl RequestObserver<UserSingInRequest, UserSingInResponse, UserSingInConclusion> for ObserverRequest {

    fn response(
        &mut self,
        request: UserSingInRequest,
        cx: &mut dyn Context,
    ) -> Result<(UserSingInResponse, UserSingInConclusion), String> {
        Ok((UserSingInResponse { error: None }, UserSingInConclusion::Accept))
    }
}

impl UserSingInObserver<UserSingInRequest, UserSingInResponse, UserSingInConclusion> for ObserverRequest {

    fn accept(
        &mut self,
        cx: &mut dyn Context,
        request: UserSingInRequest,
    ) -> Result<(), String> {
        Ok(())
    }

    fn broadcast(
        &mut self,
        cx: &mut dyn Context,
        request: UserSingInRequest,
    ) -> Result<(), String> {
        Ok(())
    }

    fn deny(
        &mut self,
        cx: &mut dyn Context,
        request: UserSingInRequest,
    ) -> Result<(), String> {
        Ok(())
    }
}