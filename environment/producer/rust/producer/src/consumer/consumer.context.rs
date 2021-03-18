use super::consumer_identification::{EFilterMatchCondition};
use super::Protocol;

pub trait Context {

    fn send(&self, buffer: Vec<u8>) -> Result<(), String>;

    fn send_to(&self, buffer: Vec<u8>, filter: Protocol::Identification, condition: EFilterMatchCondition) -> Result<(), String>;

}
