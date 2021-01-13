use super::consumer_context::{Context, Encodable};
use super::observer::{RequestObserver, RequestObserverErrors};
use super::consumer_identification::EFilterMatchCondition;
use super::{ Broadcasting };
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserSingInConclusion {
    Accept,
    Deny,
}

pub trait UserSingInObserver<
    Request: Clone,
    Response: Encodable,
    Conclusion: Eq + Hash,
>: RequestObserver<Request, Response, UserSingInConclusion>
{
    fn accept(
        &mut self,
        cx: &dyn Context,
        request: Request,
    ) -> Result<(), String>;

    fn broadcast(
        &mut self,
        cx: &dyn Context,
        request: Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String>;

    fn deny(
        &mut self,
        cx: &dyn Context,
        request: Request,
    ) -> Result<(), String>;

    fn emit(
        &mut self,
        cx: &dyn Context,
        request: Request,
        broadcast: &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        match self.response(request.clone(), cx) {
            Ok((mut response, conclusion)) => match response.abduct() {
                Ok(buffer) => {
                    if let Err(e) = cx.send(buffer) {
                        Err(RequestObserverErrors::ResponsingError(e))
                    } else {
                        match conclusion {
                            UserSingInConclusion::Accept => {
                                if let Err(e) = self.accept(cx, request.clone()) {
                                    return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                }
                                if let Err(e) = self.broadcast(cx, request, broadcast) {
                                    return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                }
                            }
                            UserSingInConclusion::Deny => {
                                if let Err(e) = self.deny(cx, request) {
                                    return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                                }
                            }
                        }
                        Ok(())
                    }
                }
                Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
            },
            Err(e) => Err(RequestObserverErrors::GettingResponseError(e)),
        }
    }
}
