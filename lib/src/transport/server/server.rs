use super::events::{ ServerEvents };
use std::sync::mpsc::{ Sender, Receiver };
use uuid::Uuid;

pub trait Server: Send {

    fn listen(&mut self, channel: Sender<ServerEvents>, messages: Receiver<(Vec<u8>, Option<Uuid>)>) -> Result<(), String>;

}
