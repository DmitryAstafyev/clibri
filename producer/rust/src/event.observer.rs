use super::context::{ Context };
use std::collections::{ HashMap };
use uuid::Uuid;

pub type EventHandler<Request, Identification> = dyn Fn(Request, &mut dyn Context<Identification>) -> Result<Vec<u8>, String>;

pub enum EventObserverErrors {
    HandlerUuidIsAlreadyExist,
    NoHandlerFound,
}

pub trait EventObserver<Request: Clone, Identification> {

    fn subscribe(&mut self, hanlder: &'static EventHandler<Request, Identification>) -> Result<Uuid, EventObserverErrors>;
    fn unsubscribe(&mut self, uuid: Uuid) -> Result<(), EventObserverErrors>;
    fn emit(&mut self, cx: &mut dyn Context<Identification>, request: Request) -> Result<(), HashMap<Uuid, String>>;

}

pub struct Observer<Request: Clone, Identification> {
    handlers: HashMap<Uuid, Box<EventHandler<Request, Identification>>>,
}

impl<Request: Clone, Identification> EventObserver<Request, Identification> for  Observer<Request, Identification> {

    fn subscribe(&mut self, hanlder: &'static EventHandler<Request, Identification>) -> Result<Uuid, EventObserverErrors> {
        let uuid: Uuid = Uuid::new_v4();
        if self.handlers.insert(uuid, Box::new(hanlder)).is_none() {
            Ok(uuid)
        } else {
            Err(EventObserverErrors::HandlerUuidIsAlreadyExist)
        }
    }

    fn unsubscribe(&mut self, uuid: Uuid) -> Result<(), EventObserverErrors> {
        if self.handlers.remove(&uuid).is_some() {
            Ok(())
        } else {
            Err(EventObserverErrors::NoHandlerFound)
        }
    }

    fn emit(&mut self, cx: &mut dyn Context<Identification>, request: Request) -> Result<(), HashMap<Uuid, String>> {
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