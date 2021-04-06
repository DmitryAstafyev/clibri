use super::consumer_identification::{Filter};
use super::{Protocol};
use uuid::Uuid;

pub trait Context {

    fn send(&self, buffer: Vec<u8>) -> Result<(), String>;

    fn send_to(&self, buffer: Vec<u8>, filter: Filter) -> Result<(), String>;

    fn uuid(&self) -> Uuid;

    fn assign(&self, assigned: Protocol::Identification::AssignedKey, overwrite: bool) -> Result<(), String>;
}
