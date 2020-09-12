use log::{ info };
use log4rs;
use std::net::TcpListener;
use fiber:: { server, session, session_context, controller, CloseFrame, Request, Response, ErrorResponse };
use session_context::{ SessionContext };
#[path = "./main.protocol.rs"]
mod protocol;

struct ServerController { }

impl controller::Controller for ServerController {

    fn handshake(&mut self, req: &Request, mut response: Response) -> Result<Response, ErrorResponse> {
        Ok(response)
    }

    fn error(&mut self, e: controller::Error) {
        info!("{:?}", e);
    }

}

struct ClientSession { }

impl session::Session<protocol::Messages> for ClientSession {

    fn connected(&mut self, mut _cx: SessionContext) -> () {
        println!("Connected: {}", _cx.get_uuid());
        ()
    }

    fn error(&mut self, _err: session::Error, mut _cx: Option<SessionContext>) -> () {
        println!("Connected: {:?}", _err);
        ()
    }

    fn disconnect(&mut self, mut _cx: SessionContext, _frame: Option<CloseFrame>) -> () {
        println!("Disconnected: {}", _cx.get_uuid());
        ()
    }

    fn message(&mut self, _msg: protocol::Messages, mut _cx: SessionContext) -> Result<(), String> {
        println!("{}:: {:?}", _cx.get_uuid(), _msg);
        Ok(())
    }

    fn text(&mut self, _text: String, mut _cx: SessionContext) -> Result<(), String> {
        println!("{}:: {:?}", _cx.get_uuid(), _text);
        Ok(())
    }
}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("Started...");
    let listener = TcpListener::bind("127.0.0.1:8088").unwrap();
    let controller: ServerController = ServerController {};
    let mut serv: server::Server<protocol::Messages> = server::Server::new(controller);
    let pro = protocol::Protocol {};
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!(">>>>>> Connection!");
                let session = ClientSession {};
                match serv.add(stream, session, pro.clone()) {
                    Ok(_) => {},
                    Err(e) => info!("Fail to add connection due error: {}", e),
                }
            },
            Err(_e) => {

            }
        }
    }
}