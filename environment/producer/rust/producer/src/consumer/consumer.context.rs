use super::consumer_identification::{Filter};
use uuid::Uuid;

pub trait Context {

    fn send(&self, buffer: Vec<u8>) -> Result<(), String>;

    fn send_to(&self, buffer: Vec<u8>, filter: Filter) -> Result<(), String>;

    fn uuid(&self) -> Uuid;

}
