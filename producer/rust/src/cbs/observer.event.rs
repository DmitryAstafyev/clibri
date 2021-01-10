use super::context::Context;
use super::broadcast_observer::BroadcastObserverErrors;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

pub type EventHandler<Request, Identification> =
    dyn (Fn(Request, &mut dyn Context<Identification>) -> Result<(), String>) + Send + Sync;

pub enum EventObserverErrors {
    HanderIsAlreadyExist,
    NoHandlerFound,
    ErrorOnHandeling(String),
    ErrorOnBroadcasting(BroadcastObserverErrors),
}

pub trait EventObserver<Request: Clone, Identification, Conclusion: Eq + Hash> {
    fn subscribe(
        &mut self,
        conclusion: Conclusion,
        hanlder: &'static EventHandler<Request, Identification>,
    ) -> Result<(), EventObserverErrors>;
    fn unsubscribe(&mut self, conclusion: Conclusion) -> Result<(), EventObserverErrors>;
    fn emit(
        &mut self,
        conclusion: Conclusion,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), EventObserverErrors>;
}

pub struct Observer<Request: Clone, Identification, Conclusion: Eq + Hash> {
    handlers: HashMap<Conclusion, Box<EventHandler<Request, Identification>>>,
}

impl<Request: Clone, Identification, Conclusion: Eq + Hash> Observer<Request, Identification, Conclusion> {

    pub fn new() -> Self {
        Observer {
            handlers: HashMap::new(),
        }    
    }
    
}

impl<Request: Clone, Identification, Conclusion: Eq + Hash>
    EventObserver<Request, Identification, Conclusion>
    for Observer<Request, Identification, Conclusion>
{

    fn subscribe(
        &mut self,
        conclusion: Conclusion,
        hanlder: &'static EventHandler<Request, Identification>,
    ) -> Result<(), EventObserverErrors> {
        if self
            .handlers
            .insert(conclusion, Box::new(hanlder))
            .is_none()
        {
            Ok(())
        } else {
            Err(EventObserverErrors::HanderIsAlreadyExist)
        }
    }

    fn unsubscribe(&mut self, conclusion: Conclusion) -> Result<(), EventObserverErrors> {
        if self.handlers.remove(&conclusion).is_some() {
            Ok(())
        } else {
            Err(EventObserverErrors::NoHandlerFound)
        }
    }

    fn emit(
        &mut self,
        conclusion: Conclusion,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), EventObserverErrors> {
        if let Some(handler) = self.handlers.get(&conclusion) {
            if let Err(e) = handler(request, cx) {
                Err(EventObserverErrors::ErrorOnHandeling(e))
            } else {
                Ok(())
            }
        } else {
            Err(EventObserverErrors::NoHandlerFound)
        }
    }
}
