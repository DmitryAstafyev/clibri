use log::{ info };
use log4rs;
use std::net::TcpListener;
use fiber:: { server, session };
use tungstenite::protocol::CloseFrame;

#[path = "./main.protocol.rs"]
mod protocol;

struct ClientSession {

}

impl session::Session<protocol::Messages> for ClientSession {

    fn connected(&mut self, mut _cx: server::Context) -> () {
        println!("Connected: {}", _cx.get_uuid());
        ()
    }

    fn error(&mut self, _err: session::Error, mut _cx: Option<server::Context>) -> () {
        println!("Connected: {:?}", _err);
        ()
    }

    fn disconnect(&mut self, mut _cx: server::Context, _frame: Option<CloseFrame>) -> () {
        println!("Disconnected: {}", _cx.get_uuid());
        ()
    }

    fn message(&mut self, _msg: protocol::Messages, mut _cx: server::Context) -> Result<(), String> {
        println!("{}:: {:?}", _cx.get_uuid(), _msg);
        Ok(())
    }

    fn text(&mut self, _text: String, mut _cx: server::Context) -> Result<(), String> {
        println!("{}:: {:?}", _cx.get_uuid(), _text);
        Ok(())
    }
}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("Started...");
    let listener = TcpListener::bind("127.0.0.1:8088").unwrap();
    let mut serv: server::Server<protocol::Messages> = server::Server::new();
    let pro = protocol::Protocol {};
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!(">>>>>> Connection!");
                let session = ClientSession {};
                serv.add(stream, session, pro.clone(), Some(|_err: session::Error| {
                    info!("Err");
                }));
            },
            Err(_e) => {

            }
        }
    }
}