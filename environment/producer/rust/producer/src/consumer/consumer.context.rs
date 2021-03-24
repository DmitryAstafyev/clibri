use super::consumer_identification::{Filter};
use super::Protocol;

pub trait Context {

    fn send(&self, buffer: Vec<u8>) -> Result<(), String>;

    fn send_to(&self, buffer: Vec<u8>, filter: Filter) -> Result<(), String>;

}
