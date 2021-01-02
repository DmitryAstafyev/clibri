use super::context::{ Context };
use std::collections::{ HashMap };

pub type EventHandler<Request, Identification> = dyn Fn(Request, &mut dyn Context<Identification>) -> Result<Vec<u8>, String>;

pub enum EventObserverErrors {
    HandlerUuidIsAlreadyExist,
    NoHandlerFound,
}

pub trait EventObserver<Request: Clone, Identification, Conclusion> {

    fn subscribe(&mut self, conclusion: Conclusion, hanlder: &'static EventHandler<Request, Identification>) -> Result<(), EventObserverErrors>;
    fn unsubscribe(&mut self, conclusion: Conclusion) -> Result<(), EventObserverErrors>;
    fn emit(&mut self, conclusion: Conclusion, cx: &mut dyn Context<Identification>, request: Request) -> Result<(), HashMap<Conclusion, String>>;

}

pub struct Observer<Request: Clone, Identification, Conclusion> {
    handlers: HashMap<Conclusion, Box<EventHandler<Request, Identification>>>,
}

impl<Request: Clone, Identification, Conclusion> EventObserver<Request, Identification, Conclusion> for  Observer<Request, Identification, Conclusion> {

    fn subscribe(&mut self, conclusion: Conclusion, hanlder: &'static EventHandler<Request, Identification>) -> Result<(), EventObserverErrors> {
        let uuid: Uuid = Uuid::new_v4();
        if self.handlers.insert(uuid, Box::new(hanlder)).is_none() {
            Ok(uuid)
        } else {
            Err(EventObserverErrors::HandlerUuidIsAlreadyExist)
        }
    }

    fn unsubscribe(&mut self, conclusion: Conclusion) -> Result<(), EventObserverErrors> {
        if self.handlers.remove(&uuid).is_some() {
            Ok(())
        } else {
            Err(EventObserverErrors::NoHandlerFound)
        }
    }

    fn emit(&mut self, conclusion: Conclusion, cx: &mut dyn Context<Identification>, request: Request) -> Result<(), HashMap<Conclusion, String>> {
        let mut errs: HashMap<Uuid, String> = HashMap::new();
        for (uuid, handler) in self.handlers.iter() {
            if let Err(e) = handler(request.clone(), cx) {
                errs.insert(*uuid, e);
            }
        }
        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }

}