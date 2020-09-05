use super::{ server };
use server::{ Context };
use tungstenite::protocol::CloseFrame;

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

    fn connected(&mut self, cx: Context) -> () {
        ()
    }
    
    fn error(&mut self, err: Error, cx: Option<Context>) -> () {
        ()
    }
    
    fn disconnect(&mut self, cx: Context, frame: Option<CloseFrame>) -> () {
        ()
    }
    
    fn message(&mut self, msg: T, cx: Context) -> Result<(), String> {
        Ok(())
    }
    
    fn text(&mut self, text: String, cx: Context) -> Result<(), String> {
        Ok(())
    }

}