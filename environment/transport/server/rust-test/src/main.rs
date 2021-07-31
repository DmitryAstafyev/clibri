use console::style;
use fiber::env::logs;
use fiber::server::{control::Control, events::Events, interface::Interface};
use fiber_transport_server::{errors::Error, server::Server};

use futures::{
    executor,
    future::join_all,
    stream::{self},
    SinkExt, StreamExt,
};
use indicatif::ProgressBar;
use log::{error, info};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;
use tokio::{
    join,
    net::TcpStream,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender, channel, Receiver, Sender},
        oneshot,
    },
    task::{spawn, JoinHandle},
    time::Duration,
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use uuid::Uuid;

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
    pub failed: u32,
    pub sent: u32,
    pub write: u32,
    pub recieved: u32,
    pub created_in: u128,
    pub sent_in: u128,
    pub done_in: u128,
}

impl Stat {
    pub fn print(&self) {
        println!("==========================================================================");
        println!("Clients created:      {}", self.created);
        println!("Clients destroyed:    {}", self.destroyed);
        println!("Clients connected:    {}", self.connected);
        println!("Clients disconnected: {}", self.disconnected);
        println!("Clients failed:       {}", self.failed);
        println!("Packages write:       {}", self.write);
        println!("Packages sent:        {}", self.sent);
        println!("Packages recieved:    {}", self.recieved);
        println!("Created in:           {}ms", self.created_in);
        println!("Sent in:              {}ms", self.sent_in);
        println!("Done in:              {}ms", self.done_in);
        println!("==========================================================================");
    }
}

//const CLIENTS: usize = 28227;
const CLIENTS: usize = 10000;

async fn connect_client() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, String> {
    match connect_async("ws://127.0.0.1:8080").await {
        Ok((ws, _)) => Ok(ws),
        Err(e) => Err(format!("{}", e)),
    }
}

async fn create_client(
    ws: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    status: Sender<ClientStatus>,
    stat: Arc<RwLock<Stat>>,
    starter: oneshot::Receiver<()>,
) {
    let ws = if let Some(ws) = ws {
        ws
    } else {
        info!(target: "test", "client: starting client");
        match connect_async("ws://127.0.0.1:8080").await {
            Ok((ws, _)) => {
                info!(target: "test", "handshake has been successfully completed");
                ws
            }
            Err(e) => {
                error!(target: "test",
                    "client [connect_async]: failed to connect: {}",
                    e
                );
                status
                    .send(ClientStatus::Err(format!(
                        "client [connect_async]: failed to connect: {}",
                        e
                    ))).await
                    .expect("ClientStatus should be sent");
                return;
            }
        }
    };
    let client_status_rd = status.clone();
    let (mut write, mut read) = ws.split();
    // TODO: stream should be closed as well
    let reader: JoinHandle<
        Result<
            futures::stream::SplitStream<
                tokio_tungstenite::WebSocketStream<
                    tokio_tungstenite::MaybeTlsStream<
                        tokio::net::TcpStream
                    >
                >,
            >,
            String,
        >,
    > = spawn(async move {
        info!(target: "test", "client: reader is created");
        if let Some(msg) = read.next().await {
            let data = msg.unwrap().into_data();
            info!(target: "test", "income data: {:?}", data);
        }
        info!(target: "test", "client: reader is destroyed");
        Ok(read)
    });
    let (tx_sender_from_client, mut rx_sender_from_client): (
        UnboundedSender<Vec<u8>>,
        UnboundedReceiver<Vec<u8>>,
    ) = unbounded_channel();
    let client_status_wr = status.clone();
    let stat_sr = stat.clone();
    let writer: JoinHandle<
        Result<
            futures::stream::SplitSink<
                tokio_tungstenite::WebSocketStream<
                    tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
                >,
                tokio_tungstenite::tungstenite::Message,
            >,
            String,
        >,
    > = spawn(async move {
        info!(target: "test", "client: writer is created");
        while let Some(buffer) = rx_sender_from_client.recv().await {
            if let Err(e) = write.send(Message::Binary(buffer)).await {
                error!(target: "test",
                    "client [writer]: fail to send data: {}",
                    e
                );
                client_status_wr
                    .send(ClientStatus::Err(format!(
                        "client [writer]: fail to send data: {}",
                        e
                    ))).await
                    .expect("ClientStatus should be sent");
                return Err(String::from("Fail to write data"));
            }
        }
        match stat_sr.write() {
            Ok(mut stat) => stat.write += 1,
            Err(_) => {
                error!(target: "test", "cannot write stat");
            }
        };
        info!(target: "test", "client: writer is destroyed");
        Ok(write)
    });
    let client_status_sd = status.clone();
    let stat_sw = stat.clone();
    let sender_from_client = spawn(async move {
        if let Err(e) = starter.await {
            error!(target: "test",
                "client [sender_from_client]: fail recieve from starter channel: {}",
                e
            );
            client_status_sd
                .send(ClientStatus::Err(format!(
                    "client [sender_from_client]: fail recieve from starter channel: {}",
                    e
                ))).await
                .expect("ClientStatus should be sent");
            return;
        }
        info!(target: "test", "client: sender is created");
        let buffer: Vec<u8> = vec![0u8, 1u8, 2u8, 3u8, 4u8];
        if let Err(e) = tx_sender_from_client.send(buffer) {
            error!(target: "test",
                "client [tx_sender_from_client]: failed to send data: {}",
                e
            );
            client_status_sd
                .send(ClientStatus::Err(format!(
                    "client [tx_sender_from_client]: failed to send data: {}",
                    e
                ))).await
                .expect("ClientStatus should be sent");
            return;
        } else {
            info!(target: "test", "client: data has been sent");
            match stat_sw.write() {
                Ok(mut stat) => stat.sent += 1,
                Err(_) => {
                    error!(target: "test", "cannot write stat");
                }
            };
        }
        info!(target: "test", "client: sender is destroyed");
    });
    match stat.write() {
        Ok(mut stat) => stat.created += 1,
        Err(_) => {
            error!(target: "test", "cannot write stat");
        }
    };
    let (read, write, _sender) = join!(reader, writer, sender_from_client,);
    match read {
        Ok(read) => match read {
            Ok(read) => {
                drop(read);
            },
            Err(err) => {
                error!(target: "test", "Fail read data form client: {}", err);
            }
        },
        Err(err) => {
            error!(target: "test", "Fail join reader: {}", err);
        }
    }
    match write {
        Ok(write) => match write {
            Ok(write) => {
                drop(write);
            },
            Err(err) => {
                error!(target: "test", "Fail write data: {}", err);
            },
        },
        Err(err) => {
            error!(target: "test", "Fail join writer: {}", err);
        },
    }
    match stat.write() {
        Ok(mut stat) => stat.destroyed += 1,
        Err(_) => {
            error!(target: "test", "cannot write stat");
        }
    };
    client_status_rd
        .send(ClientStatus::Done).await
        .expect("ClientStatus should be sent");
    info!(target: "test", "client: done");
}

async fn create_events_listener(
    mut rx_events: UnboundedReceiver<Events<Error>>,
    tx_server_state: Sender<ServerState>,
    tx_sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
    tx_server_shutdown: UnboundedSender<()>,
    stat: Arc<RwLock<Stat>>,
) {
    info!(target: "test", "starting event listener");
    while let Some(event) = rx_events.recv().await {
        match event {
            Events::Ready => {
                info!(target: "test", "[T][EventsLoop] server is ready");
                if let Err(e) = tx_server_state.send(ServerState::Ready).await {
                    error!(target: "test", "cannot send server state: {}", e);
                }
            }
            Events::Shutdown => {
                if let Err(e) = tx_server_shutdown.send(()) {
                    error!(target: "test", "cannot send server state: {}", e);
                }
            }
            Events::Connected(uuid) => {
                info!(target: "test", "[T][EventsLoop] {} connected", uuid.clone());
                match stat.write() {
                    Ok(mut stat) => stat.connected += 1,
                    Err(_) => {
                        error!(target: "test", "cannot write stat");
                    }
                };
            }
            Events::Disconnected(uuid) => {
                info!(target: "test", "[T][EventsLoop] {} disconnected", uuid);
                match stat.write() {
                    Ok(mut stat) => stat.disconnected += 1,
                    Err(_) => {
                        error!(target: "test", "cannot write stat");
                    }
                };
            }
            Events::Received(uuid, buffer) => {
                info!(target: "test",
                    "[T][EventsLoop] {} data has been received: {:?}",
                    uuid, buffer
                );
                match stat.write() {
                    Ok(mut stat) => stat.recieved += 1,
                    Err(_) => {
                        error!(target: "test", "cannot write stat");
                    }
                };
                let buffer: Vec<u8> = vec![5u8, 6u8, 7u8, 8u8, 9u8];
                if let Err(e) = tx_sender.send((buffer, Some(uuid))) {
                    error!(target: "test",
                        "fail to send data to connection {}: {}",
                        uuid, e
                    );
                } else {
                    info!(target: "test", "has been sent data to {}", uuid);
                }
            }
            Events::Error(uuid, err) => {
                error!(target: "test", "Error ({:?}): {:?}", uuid, err);
            }
            Events::ConnectionError(uuid, err) => {
                error!(target: "test", "ConnectionError ({:?}): {:?}", uuid, err);
            }
            Events::ServerError(err) => {
                error!(target: "test", "ServerError: {:?}", err);
            }
        }
    }
}

async fn connect_all() {
    info!(target: "test", "starting");
    let (tx_server_shutdown, mut rx_server_shutdown): (UnboundedSender<()>, UnboundedReceiver<()>) =
        unbounded_channel();
    let (tx_status, mut rx_status): (Sender<ClientStatus>, Receiver<ClientStatus>) =
        channel(10);
    let (tx_server_state, mut rx_server_state): (
        Sender<ServerState>,
        Receiver<ServerState>,
    ) = channel(10);
    let stat: Arc<RwLock<Stat>> = Arc::new(RwLock::new(Stat {
        created: 0,
        destroyed: 0,
        connected: 0,
        disconnected: 0,
        failed: 0,
        sent: 0,
        write: 0,
        recieved: 0,
        created_in: 0,
        sent_in: 0,
        done_in: 0,
    }));
    let mut server: Server = Server::new(String::from("127.0.0.1:8080"));
    let rx_events = match server.observer() {
        Ok(rx_events) => rx_events,
        Err(e) => panic!("{}", e),
    };
    let tx_sender = server.sender();
    let tx_control = server.control();
    spawn(async move {
        match server.listen() {
            Ok(task) => {
                if let Err(e) = task.await {
                    error!(target: "test", "fail on server task: {}", e);
                }
            }
            Err(e) => {
                error!(target: "test", "fail to create server: {}", e);
                panic!("{}", e);
            }
        }
        server.print_stat();
    });
    let stat_sr = stat.clone();
    thread::spawn(move || {
        executor::block_on(create_events_listener(
            rx_events,
            tx_server_state.clone(),
            tx_sender.clone(),
            tx_server_shutdown,
            stat_sr,
        ));
    });
    println!("{} waiting for server...", style("[test]").bold().dim(),);
    if let Some(state) = rx_server_state.recv().await {
        match state {
            ServerState::Ready => {
                println!("{} server is ready", style("[test]").bold().dim(),);
            }
        }
    } else {
        panic!("Server isn't ready");
    }
    let mut starters: Vec<oneshot::Sender<()>> = vec![];
    let mut connections = vec![];
    println!(
        "{} creating clients executors...",
        style("[test]").bold().dim(),
    );
    let start = Instant::now();
    for _ in 0..CLIENTS {
        let (tx_client_starter, rx_client_starter): (oneshot::Sender<()>, oneshot::Receiver<()>) =
        oneshot::channel();
        let client = create_client(None, tx_status.clone(), stat.clone(), rx_client_starter);
        connections.push(client);
        starters.push(tx_client_starter);
    }
    match stat.write() {
        Ok(mut stat) => stat.created_in = start.elapsed().as_millis(),
        Err(_) => {
            error!(target: "test", "cannot write stat");
        }
    };
    println!(
        "{} created {} clients",
        style("[test]").bold().dim(),
        connections.len()
    );
    let stat_ja = stat.clone();
    spawn(async move {
        println!("{} starting clients...", style("[test]").bold().dim(),);
        let len = connections.len();
        join_all(connections).await;
        println!(
            "{} {} clients were started",
            style("[test]").bold().dim(),
            len
        );
        match stat_ja.read() {
            Ok(stat) => stat.print(),
            Err(_) => {
                error!(target: "test", "cannot write stat");
            }
        };
    });
    let start = Instant::now();
    let start_wf = Instant::now();
    println!(
        "{} send messages to clients into queue",
        style("[test]").bold().dim(),
    );
    for starter in starters {
        starter.send(()).expect("Client should be started");
    }
    match stat.write() {
        Ok(mut stat) => stat.sent_in = start.elapsed().as_millis(),
        Err(_) => {
            error!(target: "test", "cannot write stat");
        }
    };
    println!(
        "{} waiting for clients are doing job...",
        style("[test]").bold().dim(),
    );
    let mut done: usize = 0;
    let pb = ProgressBar::new(CLIENTS as u64);
    while let Some(status) = rx_status.recv().await {
        //TODO: check state. it might be an error
        match status {
            ClientStatus::Done => {
                done += 1;
                pb.inc(1);
                if done == CLIENTS {
                    break;
                }
            }
            ClientStatus::Err(e) => {
                error!(target: "test", "client status error: {}", e);
                match stat.write() {
                    Ok(mut stat) => stat.failed += 1,
                    Err(_) => {
                        error!(target: "test", "cannot write stat");
                    }
                };
                pb.inc(1);
            }
        }
    }
    println!(
        "{} no more channels for clients...",
        style("[test]").bold().dim(),
    );
    pb.finish_and_clear();
    match stat.write() {
        Ok(mut stat) => stat.done_in = start_wf.elapsed().as_millis(),
        Err(_) => {
            error!(target: "test", "cannot write stat");
        }
    };
    println!(
        "{} waiting for clients are disconnecting...",
        style("[test]").bold().dim(),
    );
    let pb = ProgressBar::new(CLIENTS as u64);
    loop {
        if let Ok(stat) = stat.read() {
            if (stat.disconnected - stat.failed) == stat.connected {
                break;
            } else {
                pb.inc((stat.disconnected - stat.failed) as u64);
                //println!("Waiting for: pending = {}; connected = {}; destroyed = {};", CLIENTS as u32 - stat.disconnected, stat.connected, stat.destroyed);
            }
        } else {
            panic!("Fail to read stat");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    pb.finish_and_clear();
    println!("==========================================================================");
    if let Ok(stat) = stat.read() {
        println!("Clients created:      {}", stat.created);
        println!("Clients destroyed:    {}", stat.destroyed);
        println!("Clients connected:    {}", stat.connected);
        println!("Clients disconnected: {}", stat.disconnected);
        println!("Clients failed:       {}", stat.failed);
        println!("Packages sent:        {}", stat.sent);
        println!("Packages recieved:    {}", stat.recieved);
        println!("Created in:           {}ms", stat.created_in);
        println!("Sent in:              {}ms", stat.sent_in);
        println!("Done in:              {}ms", stat.done_in);
    };
    println!("==========================================================================");
    executor::block_on(async move {
        if let Err(e) = tx_control.send(Control::Shutdown) {
            error!(target: "test", "Fail send Control::Shutdown. Error: {}", e);
        }
        println!(
            "{} waiting for shutdown server...",
            style("[test]").bold().dim(),
        );
        rx_server_shutdown.recv().await;
        println!("{} server is shutdown", style("[test]").bold().dim(),);
    });
}

async fn connect_one_by_one() {
    info!(target: "test", "starting");
    let (tx_server_shutdown, mut rx_server_shutdown): (UnboundedSender<()>, UnboundedReceiver<()>) =
        unbounded_channel();
    let (tx_status, mut rx_status): (Sender<ClientStatus>, Receiver<ClientStatus>) =
        channel(10);
    let (tx_server_state, mut rx_server_state): (
        Sender<ServerState>,
        Receiver<ServerState>,
    ) = channel(10);
    let stat: Arc<RwLock<Stat>> = Arc::new(RwLock::new(Stat {
        created: 0,
        destroyed: 0,
        connected: 0,
        disconnected: 0,
        failed: 0,
        sent: 0,
        write: 0,
        recieved: 0,
        created_in: 0,
        sent_in: 0,
        done_in: 0,
    }));
    let mut server: Server = Server::new(String::from("127.0.0.1:8080"));
    let rx_events = match server.observer() {
        Ok(rx_events) => rx_events,
        Err(e) => panic!("{}", e),
    };
    let tx_sender = server.sender();
    let tx_control = server.control();
    spawn(async move {
        println!("{} starting server...", style("[test]").bold().dim(),);
        match server.listen() {
            Ok(task) => {
                if let Err(e) = task.await {
                    error!(target: "test", "fail on server task: {}", e);
                }
            }
            Err(e) => {
                error!(target: "test", "fail to create server: {}", e);
                panic!("{}", e);
            }
        }
        server.print_stat();
    });
    let stat_sr = stat.clone();
    thread::spawn(move || {
        executor::block_on(create_events_listener(
            rx_events,
            tx_server_state.clone(),
            tx_sender.clone(),
            tx_server_shutdown,
            stat_sr,
        ));
    });
    println!("{} waiting for server...", style("[test]").bold().dim(),);
    if let Some(state) = rx_server_state.recv().await {
        match state {
            ServerState::Ready => {
                println!("{} server is ready", style("[test]").bold().dim(),);
            }
        }
    } else {
        panic!("Server isn't started");
    }
    let mut connectings_tasks = vec![];
    println!(
        "{} creating clients connectors...",
        style("[test]").bold().dim(),
    );
    for _ in 0..CLIENTS {
        connectings_tasks.push(connect_client());
    }
    let stat_con = stat.clone();
    spawn(async move {
        println!(
            "{} starting clients connections...",
            style("[test]").bold().dim(),
        );
        let mut sockets = vec![];
        let mut stream = stream::iter(connectings_tasks);
        let pb = ProgressBar::new(CLIENTS as u64);
        while let Some(connector) = stream.next().await {
            match connector.await {
                Ok(socket) => {
                    pb.inc(1);
                    sockets.push(socket);
                }
                Err(err) => panic!("{}", err),
            };
        }
        pb.finish_and_clear();
        let mut starters: Vec<oneshot::Sender<()>> = vec![];
        let mut connections = vec![];
        println!(
            "{} creating clients executors...",
            style("[test]").bold().dim(),
        );
        let start = Instant::now();
        let pb = ProgressBar::new(CLIENTS as u64);
        for _ in 0..sockets.len() {
            let (tx_client_starter, rx_client_starter): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
            let client = create_client(
                Some(sockets.remove(0)),
                tx_status.clone(),
                stat_con.clone(),
                rx_client_starter,
            );
            connections.push(client);
            starters.push(tx_client_starter);
            pb.inc(1);
        }
        pb.finish_and_clear();
        match stat_con.write() {
            Ok(mut stat) => stat.created_in = start.elapsed().as_millis(),
            Err(_) => {
                error!(target: "test", "cannot write stat");
            }
        };
        println!("{} starting clients...", style("[test]").bold().dim(),);
        let start = Instant::now();
        println!(
            "{} send messages to clients into queue",
            style("[test]").bold().dim(),
        );
        for starter in starters {
            starter.send(()).expect("Client should be started");
        }
        match stat_con.write() {
            Ok(mut stat) => stat.sent_in = start.elapsed().as_millis(),
            Err(_) => {
                error!(target: "test", "cannot write stat");
            }
        };
        join_all(connections).await;
    });
    let start_wf = Instant::now();
    println!(
        "{} waiting for clients are doing job...",
        style("[test]").bold().dim(),
    );
    let mut done: usize = 0;
    let pb = ProgressBar::new(CLIENTS as u64);
    while let Some(status) = rx_status.recv().await {
        //TODO: check state. it might be an error
        match status {
            ClientStatus::Done => {
                done += 1;
                pb.inc(1);
                if done == CLIENTS {
                    break;
                }
            }
            ClientStatus::Err(e) => {
                error!(target: "test", "client status error: {}", e);
                match stat.write() {
                    Ok(mut stat) => stat.failed += 1,
                    Err(_) => {
                        error!(target: "test", "cannot write stat");
                    }
                };
                pb.inc(1);
            }
        }
    }
    pb.finish_and_clear();
    println!(
        "{} no more channels for clients...",
        style("[test]").bold().dim(),
    );
    match stat.write() {
        Ok(mut stat) => stat.done_in = start_wf.elapsed().as_millis(),
        Err(_) => {
            error!(target: "test", "cannot write stat");
        }
    };
    println!(
        "{} waiting for clients are disconnecting...",
        style("[test]").bold().dim(),
    );
    let pb = ProgressBar::new(CLIENTS as u64);
    loop {
        if let Ok(stat) = stat.read() {
            if (stat.disconnected - stat.failed) == stat.connected {
                break;
            } else {
                pb.inc((stat.disconnected - stat.failed) as u64);
                //println!("Waiting for: pending = {}; connected = {}; destroyed = {};", CLIENTS as u32 - stat.disconnected, stat.connected, stat.destroyed);
            }
        } else {
            panic!("Fail to read stat");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    pb.finish_and_clear();
    println!("==========================================================================");
    if let Ok(stat) = stat.read() {
        println!("Clients created:      {}", stat.created);
        println!("Clients destroyed:    {}", stat.destroyed);
        println!("Clients connected:    {}", stat.connected);
        println!("Clients disconnected: {}", stat.disconnected);
        println!("Clients failed:       {}", stat.failed);
        println!("Packages sent:        {}", stat.sent);
        println!("Packages recieved:    {}", stat.recieved);
        println!("Created in:           {}ms", stat.created_in);
        println!("Sent in:              {}ms", stat.sent_in);
        println!("Done in:              {}ms", stat.done_in);
    };
    println!("==========================================================================");
    executor::block_on(async move {
        if let Err(e) = tx_control.send(Control::Shutdown) {
            error!(target: "test", "Fail send Control::Shutdown. Error: {}", e);
        }
        println!(
            "{} waiting for shutdown server...",
            style("[test]").bold().dim(),
        );
        rx_server_shutdown.recv().await;
        println!("{} server is shutdown", style("[test]").bold().dim(),);
    });
}

#[tokio::main]
async fn main() -> Result<(), String> {
    logs::init();
    println!(
        "{} Test #1. Connect clients all together",
        style("[test #1: start]").bold().dim(),
    );
    connect_all().await;
    println!("{} Done", style("[test #1: done]").bold().dim(),);
    println!(
        "{} Test #2. Connect clients one by one",
        style("[test #2: start]").bold().dim(),
    );
    connect_one_by_one().await;
    println!("{} Done", style("[test #2: done]").bold().dim(),);
    Ok(())
}
