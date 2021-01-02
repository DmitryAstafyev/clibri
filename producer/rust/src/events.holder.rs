use super::context::{ Context };
use std::collections::{ HashMap };
use uuid::Uuid;

pub trait EventsHolder<Request: Clone, Identification> {

    fn emit(&mut self, cx: &mut dyn Context<Identification>, request: Request) -> Result<(), HashMap<Uuid, String>>;

}