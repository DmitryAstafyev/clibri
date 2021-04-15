use super::events::{ Events };
use uuid::Uuid;
use async_channel::{
    Sender,
    Receiver
};
pub trait Interface: Send {

    fn listen(&'static mut self, channel: Sender<Events>, messages: Receiver<(Vec<u8>, Option<Uuid>)>) -> Result<(), String>;

}
