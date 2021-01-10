use super::context::{ Context, Encodable };
use super::event_observer::EventObserverErrors;
use super::events_holder::EventsHolder;
use std::cmp::Eq;
use std::hash::Hash;

pub type RequestHandler<Request, Response: Encodable, Identification, Conclusion: Eq + Hash> =
    dyn (Fn(Request, &mut dyn Context<Identification>) -> Result<(Response, Conclusion), String>) + Send + Sync;

pub enum RequestObserverErrors {
    AlreadySubscribed,
    AlreadyUnsubscrided,
    NoConnectionToResponse,
    ResponsingError(String),
    GettingResponseError(String),
    EncodingResponseError(String),
    NoHandlerForRequest,
    ErrorOnEventsEmit(EventObserverErrors),
}

pub trait RequestObserver<Request: Clone, Response: Encodable, Identification, Conclusion: Eq + Hash> {
    fn subscribe(
        &mut self,
        hanlder: &'static RequestHandler<Request, Response, Identification, Conclusion>,
    ) -> Result<(), RequestObserverErrors>;
    fn unsubscribe(&mut self) -> Result<(), RequestObserverErrors>;
    fn emit(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), RequestObserverErrors>;
}

pub struct Observer<Request: Clone, Response: Encodable, Identification, Conclusion: Eq + Hash, E>
where
    E: EventsHolder<Request, Identification, Conclusion> + Sized,
{
    handler: Option<Box<RequestHandler<Request, Response, Identification, Conclusion>>>,
    pub events: E,
}

impl<Request: Clone, Response: Encodable, Identification, Conclusion: Eq + Hash, E>
    Observer<Request, Response, Identification, Conclusion, E>
where
    E: EventsHolder<Request, Identification, Conclusion> + Sized,
{
    pub fn new(events: E) -> Self {
        Observer {
            handler: None,
            events,
        }
    }
}

impl<Request: Clone, Response: Encodable, Identification, Conclusion: Eq + Hash, E>
    RequestObserver<Request, Response, Identification, Conclusion> 
    for Observer<Request, Response, Identification, Conclusion, E>
where
    E: EventsHolder<Request, Identification, Conclusion> + Sized,
{
    fn subscribe(
        &mut self,
        hanlder: &'static RequestHandler<Request, Response, Identification, Conclusion>,
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
                Ok((mut msg, conclusion)) => match msg.abduct() {
                    Ok(buffer) => if let Err(e) = cx.send(buffer) {
                        Err(RequestObserverErrors::ResponsingError(e))
                    } else if let Err(e) = self.events.emit(conclusion, cx, request) {
                        Err(RequestObserverErrors::ErrorOnEventsEmit(e))
                    } else {
                        Ok(())
                    },
                    Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                },
                Err(e) => Err(RequestObserverErrors::GettingResponseError(e)),
            }
        } else {
            Err(RequestObserverErrors::NoHandlerForRequest)
        }
    }

}
