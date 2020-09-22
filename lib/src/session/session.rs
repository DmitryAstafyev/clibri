use super::{ session_context };
use session_context::{ SessionContext };
use super::CloseFrame;

#[derive(Debug, Clone)]
pub enum Error {
    Parsing(String),
    ReadSocket(String),
    Socket(String),
    Connection(String),
    Session(String),
    Channel(String),
}

#[allow(unused_variables)]
pub trait Session<T>: Send + Sync {

    fn connected(&mut self, cx: SessionContext) {
    }
    
    fn error(&mut self, err: Error, cx: Option<SessionContext>) {
    }
    
    fn disconnect(&mut self, cx: SessionContext, frame: Option<CloseFrame>) {
    }
    
    fn message(&mut self, msg: T, cx: SessionContext) -> Result<(), String> {
        Ok(())
    }
    
    fn text(&mut self, text: String, cx: SessionContext) -> Result<(), String> {
        Ok(())
    }

}