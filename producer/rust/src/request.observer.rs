use super::context::Context;
use super::events_holder::EventsHolder;
use super::event_observer::EventObserverErrors;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use uuid::Uuid;

pub trait Response {
    fn get_buffer(&mut self) -> Vec<u8>;
}

pub type RequestHandler<Request, Identification, Conclusion: Eq + Hash> =
    dyn Fn(Request, &mut dyn Context<Identification>) -> Result<(Vec<u8>, Conclusion), String>;

pub enum RequestObserverErrors {
    AlreadySubscribed,
    AlreadyUnsubscrided,
    NoConnectionTpResponse,
    ResponsingError(String),
    GettingResponseError(String),
    NoHandlerForRequest,
    ErrorOnEventsEmit(EventObserverErrors),
}

pub trait RequestObserver<Request: Clone, Identification, Conclusion: Eq + Hash> {
    fn subscribe(
        &mut self,
        hanlder: &'static RequestHandler<Request, Identification, Conclusion>,
    ) -> Result<(), RequestObserverErrors>;
    fn unsubscribe(&mut self) -> Result<(), RequestObserverErrors>;
    fn emit(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), RequestObserverErrors>;
}

pub struct Observer<Request: Clone, Identification, Conclusion: Eq + Hash> {
    handler: Option<Box<RequestHandler<Request, Identification, Conclusion>>>,
    pub events: dyn EventsHolder<Request, Identification, Conclusion>,
}

impl<Request: Clone, Identification, Conclusion: Eq + Hash>
    RequestObserver<Request, Identification, Conclusion>
    for Observer<Request, Identification, Conclusion>
{
    fn subscribe(
        &mut self,
        hanlder: &'static RequestHandler<Request, Identification, Conclusion>,
    ) -> Result<(), RequestObserverErrors> {
        if self.handler.is_some() {
            Err(RequestObserverErrors::AlreadySubscribed)
        } else {
            self.handler = Some(Box::new(hanlder));
            Ok(())
        }
    }

    fn unsubscribe(&mut self) -> Result<(), RequestObserverErrors> {
        if self.handler.is_none() {
            Err(RequestObserverErrors::AlreadyUnsubscrided)
        } else {
            self.handler = None;
            Ok(())
        }
    }

    fn emit(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), RequestObserverErrors> {
        if let Some(handler) = &self.handler {
            match handler(request.clone(), cx) {
                Ok((buffer, conclusion)) => {
                    if let Some(conn) = cx.connection() {
                        if let Err(e) = conn.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else if let Err(e) = self.events.emit(conclusion, cx, request) {
                            Err(RequestObserverErrors::ErrorOnEventsEmit(e))
                        } else {
                            Ok(())
                        }
                    } else {
                        Err(RequestObserverErrors::NoConnectionTpResponse)
                    }
                }
                Err(e) => Err(RequestObserverErrors::GettingResponseError(e)),
            }
        } else {
            Err(RequestObserverErrors::NoHandlerForRequest)
        }
    }
}
