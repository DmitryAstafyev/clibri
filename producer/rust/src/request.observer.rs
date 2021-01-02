use super::context::{ Context };
use super::events_holder::EventsHolder;
use std::collections::{ HashMap };
use uuid::Uuid;

pub trait Response {

    fn get_buffer(&mut self) -> Vec<u8>;

}

pub type RequestHandler<Request, Identification> = dyn Fn(Request, &mut dyn Context<Identification>) -> Result<Vec<u8>, String>;

pub enum RequestObserverErrors {
    AlreadySubscribed,
    AlreadyUnsubscrided,
    NoConnectionTpResponse,
    ResponsingError(String),
    GettingResponseError(String),
    NoHandlerForRequest,
    ErrorOnEventsEmit(HashMap<Uuid, String>),
}

pub trait RequestObserver<Request: Clone, Identification> {

    fn subscribe(&mut self, hanlder: &'static RequestHandler<Request, Identification>) -> Result<(), RequestObserverErrors>;
    fn unsubscribe(&mut self) -> Result<(), RequestObserverErrors>;
    fn emit(&mut self, cx: &mut dyn Context<Identification>, request: Request) -> Result<(), RequestObserverErrors>;

}

pub struct Observer<Request: Clone, Identification> {
    handler: Option<Box<RequestHandler<Request, Identification>>>,
    pub events: dyn EventsHolder<Request, Identification>,
}

impl<Request: Clone, Identification> RequestObserver<Request, Identification> for  Observer<Request, Identification> {

    fn subscribe(&mut self, hanlder: &'static RequestHandler<Request, Identification>) -> Result<(), RequestObserverErrors> {
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

    fn emit(&mut self, cx: &mut dyn Context<Identification>, request: Request) -> Result<(), RequestObserverErrors> {
        if let Some(handler) = &self.handler {
            match handler(request.clone(), cx) {
                Ok(buffer) => {
                    if let Some(conn) = cx.connection() {
                        if let Err(e) = conn.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else if let Err(errs) = self.events.emit(cx, request) {
                            Err(RequestObserverErrors::ErrorOnEventsEmit(errs))
                        } else {
                            Ok(())
                        }
                    } else {
                        Err(RequestObserverErrors::NoConnectionTpResponse)
                    }
                },
                Err(e) => Err(RequestObserverErrors::GettingResponseError(e)),
            }
        } else {
            Err(RequestObserverErrors::NoHandlerForRequest)
        }
    }

}