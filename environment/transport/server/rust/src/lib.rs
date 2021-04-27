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

#[path = "./server.stat.rs"]
pub mod stat;

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
        pub static ref logger: DefaultLogger = DefaultLogger::new("Server".to_owned(), Some(2 /* 5 VERBOSE */));
    }

}

mod test {
    use super::server::Server;
    use super::tools;
    use std::sync::{Arc, RwLock};
    use fiber::{
        logger::Logger,
        server::{errors::Errors, events::Events, interface::Interface, control::Control},
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

    struct Stat {
        pub created: u32,
        pub destroyed: u32,
        pub connected: u32,
        pub disconnected: u32,
        pub sent: u32,
        pub recieved: u32,
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
        let (tx_control, mut rx_control): (
            UnboundedSender<Control>,
            UnboundedReceiver<Control>,
        ) = unbounded_channel();
        let (tx_status, mut rx_status): (
            mpsc::Sender<ClientStatus>,
            mpsc::Receiver<ClientStatus>,
        ) = mpsc::channel();
        let (tx_server_state, mut rx_server_state): (
            mpsc::Sender<ServerState>,
            mpsc::Receiver<ServerState>,
        ) = mpsc::channel();
        let mut stat: Arc<RwLock<Stat>> = Arc::new(RwLock::new(Stat {
            created: 0,
            destroyed: 0,
            connected: 0,
            disconnected: 0,
            sent: 0,
            recieved: 0
        }));
        thread::spawn(move || {
            executor::block_on(async move {
                tools::logger.verb("[T] starting server");
                let mut server: Server = Server::new(String::from("127.0.0.1:8080"));
                if let Err(e) = server.listen(tx_events, rx_sender, Some(rx_control)) {
                    tools::logger.verb(&format!("[T] fail to create server: {}", e));
                }
                server.print_stat();
            });
        });
        let stat_sr = stat.clone();
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
                            match stat_sr.write() {
                                Ok(mut stat) => stat.connected += 1,
                                Err(e) => { tools::logger.err("[T] cannot write stat"); },
                            };
                        },
                        Events::Disconnected(uuid) => {
                            tools::logger.verb(&format!("[T][EventsLoop] {} disconnected", uuid));
                            match stat_sr.write() {
                                Ok(mut stat) => stat.disconnected += 1,
                                Err(e) => { tools::logger.err("[T] cannot write stat"); },
                            };
                        },
                        Events::Received(uuid, buffer) => {
                            tools::logger.verb(&format!("[T][EventsLoop] {} data has been received: {:?}", uuid, buffer));
                            match stat_sr.write() {
                                Ok(mut stat) => stat.recieved += 1,
                                Err(e) => { tools::logger.err("[T] cannot write stat"); },
                            };
                            let buffer: Vec<u8> = vec![5u8, 6u8, 7u8, 8u8, 9u8];
                            if let Err(e) = tx_sender.send((buffer, Some(uuid.clone()))) {
                                tools::logger.err(&format!("[T] fail to send data to connection {}", uuid));
                            } else {
                                tools::logger.verb(&format!("[T] has been sent data to {}", uuid));
                            }
                        },
                        Events::Error(uuid, err) => {
                            tools::logger.err(&format!("[T] Error ({:?}): {:?}", uuid, err));
                        },
                        Events::ConnectionError(uuid, err) => {
                            tools::logger.err(&format!("[T] ConnectionError ({:?}): {:?}", uuid, err));
                        },
                        Events::ServerError(err) => {
                            tools::logger.err(&format!("[T] ServerError: {:?}", err));
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
        let client_factory = || -> mpsc::Sender<()> {
            let client_status = tx_status.clone();
            let stat_cl = stat.clone();
            let (tx_client_starter, mut rx_client_starter): (
                mpsc::Sender<()>,
                mpsc::Receiver<()>,
            ) = mpsc::channel();
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
                            client_status.send(ClientStatus::Err(tools::logger.verb(&format!("[T] client: failed to connect: {}", e)))).expect("ClientStatus should be sent");
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
                    let stat = stat_cl.clone();
                    let sender_from_client = spawn(async move {
                        if let Err(e) = rx_client_starter.recv() {
                            client_status_sd.send(ClientStatus::Err(tools::logger.verb(&format!("[T] client: fail recieve from starter channel: {}", e)))).expect("ClientStatus should be sent");
                            return;
                        }
                        tools::logger.verb("[T] client: sender is created");
                        let buffer: Vec<u8> = vec![0u8, 1u8, 2u8, 3u8, 4u8];
                        if let Err(e) = tx_sender_from_client.send(buffer) {
                            client_status_sd.send(ClientStatus::Err(tools::logger.verb(&format!("[T] client: failed to send data: {}", e)))).expect("ClientStatus should be sent");
                            return;
                        } else {
                            tools::logger.verb("[T] client: data has been sent");
                            match stat.write() {
                                Ok(mut stat) => stat.sent += 1,
                                Err(e) => { tools::logger.err("[T] cannot write stat"); },
                            };
                        }
                        rx_shutdown_sender.recv().await;
                        tools::logger.verb("[T] client: sender is destroyed");
                    });
                    match stat_cl.write() {
                        Ok(mut stat) => stat.created += 1,
                        Err(e) => { tools::logger.err("[T] cannot write stat"); },
                    };
                    select! {
                        _ = reader => { }
                        _ = writer => { }
                        _ = sender_from_client => { }
                    };
                    match stat_cl.write() {
                        Ok(mut stat) => stat.destroyed += 1,
                        Err(e) => { tools::logger.err("[T] cannot write stat"); },
                    };
                    client_status_rd.send(ClientStatus::Done).expect("ClientStatus should be sent");
                    tools::logger.verb("[T] client: done");
                });
                Ok(())
            });
            tx_client_starter
        };
        let mut starters: Vec<mpsc::Sender<()>> = vec![];
        let clients: u32 = 2;
        for _ in 0..clients {
            // std::thread::sleep(Duration::from_millis(1000));
            starters.push(client_factory());
        }

        for starter in starters {
            starter.send(()).expect("Client should be started");
        }
        let mut done: u32 = 0;
        while let Ok(msg) = rx_status.recv() {
            done += 1;
            if done == clients {
                break;
            }
        }
        // std::thread::sleep(Duration::from_millis(500));

        println!("==========================================================================");
        match stat.read() {
            Ok(stat) => {
                println!("Clients created:      {}", stat.created);
                println!("Clients destroyed:    {}", stat.destroyed);
                println!("Clients connected:    {}", stat.connected);
                println!("Clients disconnected: {}", stat.disconnected);
                println!("Packages sent:        {}", stat.sent);
                println!("Packages recieved:    {}", stat.recieved);
            },
            Err(e) => {}
        };
        println!("==========================================================================");
        std::thread::sleep(Duration::from_millis(1000));
        thread::spawn(move || {
            executor::block_on(async move {
                tx_control.send(Control::Shutdown);
            });
        });
        std::thread::sleep(Duration::from_millis(1000));

        Ok(())
    }

}

