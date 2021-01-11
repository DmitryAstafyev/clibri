use super::context::{ ConnectionContext };
use super::events::{ ServerEvents };
use std::sync::mpsc::{ Sender };

pub trait Server<T> where T: ConnectionContext + Send + Sync {

    fn listen(&mut self, channel: Sender<ServerEvents<T>>) -> Result<(), String>;

}