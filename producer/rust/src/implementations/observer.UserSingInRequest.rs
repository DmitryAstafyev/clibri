use super::context::{Context, Encodable};
use super::observer::{ RequestObserver };
use super::DeclUserSingInRequest::{ UserSingInObserver, UserSingInConclusion };
use super::{ Identification };

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

impl RequestObserver<UserSingInRequest, UserSingInResponse, Identification, UserSingInConclusion> for ObserverRequest {

    fn response(
        &mut self,
        request: UserSingInRequest,
        cx: &mut dyn Context<Identification>,
    ) -> Result<(UserSingInResponse, UserSingInConclusion), String> {
        Ok((UserSingInResponse { error: None }, UserSingInConclusion::Accept))
    }
}

impl UserSingInObserver<UserSingInRequest, UserSingInResponse, Identification, UserSingInConclusion> for ObserverRequest {

    fn accept(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: UserSingInRequest,
    ) -> Result<(), String> {
        Ok(())
    }

    fn broadcast(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: UserSingInRequest,
    ) -> Result<(), String> {
        Ok(())
    }

    fn deny(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: UserSingInRequest,
    ) -> Result<(), String> {
        Ok(())
    }
}

/*
pub trait RequestObserverA<
    Request: Clone,
    Response: Encodable,
    Identification,
    Conclusion: Eq + Hash,
>
{
    fn response(
        &mut self,
        request: Request,
        cx: &mut dyn Context<Identification>,
        conclusion: Conclusion,
    ) -> Result<Response, String>;

    fn emit(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), RequestObserverErrors>;
}

pub trait ObserverA<Request: Clone, Response: Encodable, Identification, Conclusion: Eq + Hash>:
RequestObserverA<Request, Response, Identification, UserSingInConclusion>
{

    fn add_user(
        &mut self,
        request: Request,
        cx: &mut dyn Context<Identification>,
    ) -> Result<UserSingInConclusion, String>;

    fn accept(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), String>;

    fn broadcast(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), String>;

    fn deny(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), String>;

    fn emit(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), RequestObserverErrors> {
        match self.add_user(request.clone(), cx) {
            Ok(conclusion) => match self.response(request.clone(), cx, conclusion) {
                Ok(mut msg) => match msg.abduct() {
                    Ok(buffer) => {
                        if let Err(e) = cx.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else {
                            match conclusion {
                                UserSingInConclusion::Accept => {
                                    if let Err(e) = self.accept(cx, request) {
                                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e))
                                    }
                                    if let Err(e) = self.broadcast(cx, request) {
                                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e))
                                    }
                                },
                                UserSingInConclusion::Deny => {
                                    if let Err(e) = self.deny(cx, request) {
                                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e))
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                    Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                },
                Err(e) => Err(RequestObserverErrors::GettingResponseError(e)),
            },
            Err(e) => Err(RequestObserverErrors::BeforeResponseActionFail(e))
        }
    }
}
*/

