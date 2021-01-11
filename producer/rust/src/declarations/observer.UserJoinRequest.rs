use super::consumer_context::{Context, Encodable};
use super::observer::{ConfirmedRequestObserver, RequestObserverErrors};
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserJoinConclusion {
    Accept,
    Deny,
}

pub trait UserJoinObserver<
    Request: Clone,
    Response: Encodable,
    Conclusion: Eq + Hash,
>: ConfirmedRequestObserver<Request, Response, UserJoinConclusion>
{
    fn accept(
        &mut self,
        cx: &mut dyn Context,
        request: Request,
    ) -> Result<(), String>;

    fn broadcast(
        &mut self,
        cx: &mut dyn Context,
        request: Request,
    ) -> Result<(), String>;

    fn deny(
        &mut self,
        cx: &mut dyn Context,
        request: Request,
    ) -> Result<(), String>;

    fn emit(
        &mut self,
        cx: &mut dyn Context,
        request: Request,
    ) -> Result<(), RequestObserverErrors> {
        match self.conclusion(request.clone(), cx) {
            Ok(conclusion) => match self.response(request.clone(), cx, conclusion.clone()) {
                Ok(mut response) => match response.abduct() {
                    Ok(buffer) => {
                        if let Err(e) = cx.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else {
                            match conclusion {
                                UserJoinConclusion::Accept => {
                                    if let Err(e) = self.accept(cx, request.clone()) {
                                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                    }
                                    if let Err(e) = self.broadcast(cx, request) {
                                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                    }
                                },
                                UserJoinConclusion::Deny => {
                                    if let Err(e) = self.deny(cx, request) {
                                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                    }
                                },
                            }
                            Ok(())
                        }
                    }
                    Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                },
                Err(e) => Err(RequestObserverErrors::GettingResponseError(e)),
            },
            Err(e) => Err(RequestObserverErrors::GettingConclusionError(e))
        }
    }
}
