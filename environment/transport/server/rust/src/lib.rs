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
        select,
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

    #[derive(Debug, Clone)]
    enum ServerState {
        Ready,
    }

    #[derive(Debug, Clone)]
    enum ClientStatus {
        Done,
        Err(String),
    }

    #[test]
    fn create_server() -> Result<(), String> {
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
            mpsc::Sender<ClientStatus>,
            mpsc::Receiver<ClientStatus>,
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
                            tools::logger.verb(&format!("[T][EventsLoop] {} connected", uuid.clone()));
                            
                        },
                        Events::Disconnected(uuid) => {
                            tools::logger.verb(&format!("[T][EventsLoop] {} disconnected", uuid));
                        },
                        Events::Received(uuid, buffer) => {
                            tools::logger.verb(&format!("[T][EventsLoop] {} data has been received", uuid));
                            let buffer: Vec<u8> = vec![5u8, 6u8, 7u8, 8u8, 9u8];
                            if let Err(e) = tx_sender.send((buffer, Some(uuid.clone()))) {
                                tools::logger.err(&format!("[T] fail to send data to connection {}", uuid));
                            } else {
                                tools::logger.err(&format!("[T] has been sent data to {}", uuid));
                            }
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
                        client_status.send(ClientStatus::Err(tools::logger.verb("[T] client: failed to connect"))).expect("ClientStatus should be sent");
                        return;
                    }
                };
                tools::logger.verb("[T] handshake has been successfully completed");
                let (tx_shutdown_writer, mut rx_shutdown_writer): (
                    UnboundedSender<()>,
                    UnboundedReceiver<()>,
                ) = unbounded_channel();
                let (tx_shutdown_sender, mut rx_shutdown_sender): (
                    UnboundedSender<()>,
                    UnboundedReceiver<()>,
                ) = unbounded_channel();
                let client_status_rd = client_status.clone();
                let (mut write, mut read) = ws_stream.split();
                let reader = spawn(async move {
                    tools::logger.verb("[T] client: reader is created");
                    while let Some(msg) = read.next().await {
                        let data = msg.unwrap().into_data();
                        tools::logger.verb(&format!("[T] income data: {:?}", data));
                        break;
                    }
                    tx_shutdown_writer.send(()).expect("Shutdown writer should be sent");
                    tx_shutdown_sender.send(()).expect("Shutdown sender should be sent");
                    tools::logger.verb("[T] client: reader is destroyed");
                    client_status_rd.send(ClientStatus::Done).expect("ClientStatus should be sent");
                });
                let (tx_sender_from_client, mut rx_sender_from_client): (
                    UnboundedSender<Vec<u8>>,
                    UnboundedReceiver<Vec<u8>>,
                ) = unbounded_channel();
                let client_status_wr = client_status.clone();
                let writer = spawn(async move {
                    tools::logger.verb("[T] client: writer is created");
                    while let Some(buffer) = rx_sender_from_client.recv().await {
                        if let Err(e) = write.send(Message::Binary(buffer)).await {
                            client_status_wr.send(ClientStatus::Err(tools::logger.verb(&format!("[T] client: fail to send data: {}", e)))).expect("ClientStatus should be sent");
                            return;
                        }
                    }
                    rx_shutdown_writer.recv().await;
                    tools::logger.verb("[T] client: writer is destroyed");
                });
                let client_status_sd = client_status.clone();
                let sender_from_client = spawn(async move {
                    tools::logger.verb("[T] client: sender is created");
                    let buffer: Vec<u8> = vec![0u8, 1u8, 2u8, 3u8, 4u8];
                    if let Err(e) = tx_sender_from_client.send(buffer) {
                        client_status_sd.send(ClientStatus::Err(tools::logger.verb(&format!("[T] client: failed to send data: {}", e)))).expect("ClientStatus should be sent");
                        return;
                    } else {
                        tools::logger.verb("[T] client: data has been sent");
                    }
                    rx_shutdown_sender.recv().await;
                    tools::logger.verb("[T] client: sender is destroyed");
                });
                select! {
                    _ = reader => { }
                    _ = writer => { }
                    _ = sender_from_client => { }
                };
                tools::logger.verb("[T] client: done");
            });
            Ok(())
        });
        
        while let Ok(msg) = rx_status.recv() {
            println!("{:?}", msg);
            return Ok(());
        }
        Ok(())
    }

}

