#[macro_use]
extern crate lazy_static;

pub use tokio_tungstenite::{
    tungstenite::{
        handshake::server::{
            Request,
            Response,
            ErrorResponse,
        },
        protocol::frame::coding::CloseCode
    }
};

#[path = "./server.rs"]
pub mod server;

#[path = "./connection.handshake.rs"]
pub mod handshake;

#[path = "./connection.rs"]
pub mod connection;

#[path = "./connection.channel.rs"]
pub mod channel;

#[allow(non_upper_case_globals)]
pub mod tools {
    use fiber::logger::{ DefaultLogger };

    lazy_static! {
        pub static ref logger: DefaultLogger = DefaultLogger::new("Server".to_owned(), Some(5 /* VERBOSE */));
    }

}

mod test {
    use super::server::Server;
    use super::tools;
    use fiber::{
        logger::Logger,
        server::{errors::Errors, events::Events, interface::Interface},
    };
    use tokio::{
        io::{
            AsyncRead,
            AsyncWrite,
        },
        net::{TcpListener, TcpStream},
        sync::mpsc::{unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender},
        join,
        runtime::{Runtime},
        task::spawn,
        time::{sleep, Duration},
    };
    pub use tokio_tungstenite::{
        connect_async,
        tungstenite::Message,
    };
    use uuid::Uuid;
    use std::thread;
    use std::sync::mpsc;
    use futures::{StreamExt, SinkExt, executor };

    enum ServerState {
        Ready,
    }

    #[test]
    fn create_server() {
        tools::logger.verb("[T] starting");
        let (tx_events, mut rx_events): (
            UnboundedSender<Events>,
            UnboundedReceiver<Events>,
        ) = unbounded_channel();
        let (tx_sender, mut rx_sender): (
            UnboundedSender<(Vec<u8>, Option<Uuid>)>,
            UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
        ) = unbounded_channel();
        let (tx_status, mut rx_status): (
            mpsc::Sender<Result<(), String>>,
            mpsc::Receiver<Result<(), String>>,
        ) = mpsc::channel();
        let (tx_server_state, mut rx_server_state): (
            mpsc::Sender<ServerState>,
            mpsc::Receiver<ServerState>,
        ) = mpsc::channel();
        thread::spawn(move || {
            executor::block_on(async move {
                tools::logger.verb("[T] starting server");
                let mut server: Server = Server::new(String::from("127.0.0.1:8080"));
                if let Err(e) = server.listen(tx_events, rx_sender) {
                    tools::logger.verb(&format!("[T] fail to create server: {}", e));
                }
            });
        });
        thread::spawn(move || {
            executor::block_on(async move {
                tools::logger.verb("[T] starting event listener");
                while let Some(event) = rx_events.recv().await {
                    match event {
                        Events::Ready => {
                            tools::logger.verb("[T][EventsLoop] server is ready");
                            tx_server_state.send(ServerState::Ready);
                        },
                        Events::Connected(uuid) => {
                            tools::logger.verb(&format!("[T][EventsLoop] {} connected", uuid));
                        },
                        Events::Disconnected(uuid) => {
                            tools::logger.verb(&format!("[T][EventsLoop] {} disconnected", uuid));
                        },
                        Events::Received(uuid, buffer) => {

                        },
                        Events::Error(uuid, err) => {

                        },
                        Events::ConnectionError(uuid, err) => {

                        },
                        Events::ServerError(err) => {

                        },
                    }
                }
            });
        });
        tools::logger.verb("[T] Waiting for server");
        match rx_server_state.recv() {
            Ok(state) => match state {
                ServerState::Ready => {
                    tools::logger.verb("[T] Server is ready");
                },
            },
            Err(e) => panic!(e)
        };

        let client_status = tx_status.clone();
        thread::spawn(move || {
            let rt  = match Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    return Err(tools::logger.err(&format!("Fail to create runtime executor. Error: {}", e)))
                },
            };
            rt.block_on(async move {
                tools::logger.verb("[T] client: starting client");
                let (ws_stream, _) = match connect_async("ws://127.0.0.1:8080").await {
                    Ok(res) => res,
                    Err(e) => {
                        client_status.send(Err(tools::logger.verb("[T] client: failed to connect")));
                        panic!(tools::logger.verb("[T] client: failed to connect"));
                    }
                };
                tools::logger.verb("[T] handshake has been successfully completed");
                let (mut write, read) = ws_stream.split();
                let reader = spawn(async move {
                    tools::logger.verb("[T] client: reader is created");
                    read.for_each(|message| async {
                        let data = message.unwrap().into_data();
                        tools::logger.verb(&format!("[T] income date: {:?}", data));
                    });
                    tools::logger.verb("[T] client: reader is destroyed");
                });
                let (tx_sender, mut rx_sender): (
                    UnboundedSender<Vec<u8>>,
                    UnboundedReceiver<Vec<u8>>,
                ) = unbounded_channel();
                let writer = spawn(async move {
                    tools::logger.verb("[T] client: writer is created");
                    while  let Some(buffer) = rx_sender.recv().await {
                        if let Err(e) = write.send(Message::Binary(buffer)).await {
                            panic!(tools::logger.verb("[T] client: fail to send data"))
                        }
                    }
                    tools::logger.verb("[T] client: writer is destroyed");
                });
                sleep(Duration::from_secs(1)).await;
                let buffer: Vec<u8> = vec![0u8, 1u8, 2u8, 3u8];
                if let Err(e) = tx_sender.send(buffer) {
                    client_status.send(Err(tools::logger.verb("[T] client: failed to send data")));
                    panic!(tools::logger.verb("[T] client: failed to send data"));
                } else {
                    tools::logger.verb("[T] client: data has been sent");
                }
                sleep(Duration::from_secs(1)).await;
                tools::logger.verb("[T] client: done");
            });
            // executor::block_on(async move {
            //     tools::logger.verb("[T] starting client");
            //     let (ws_stream, _) = connect_async("ws://127.0.0.1:8080").await.expect("Failed to connect");

            // });
            Ok(())
        });
        
        while let Ok(msg) = rx_status.recv() {
             println!("....");
        }
    }

}

