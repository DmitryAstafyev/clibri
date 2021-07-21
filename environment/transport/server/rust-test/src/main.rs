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
use std::sync::mpsc;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;
use tokio::{
    join,
    net::TcpStream,
    runtime::Runtime,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
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
    pub recieved: u32,
    pub created_in: u128,
    pub sent_in: u128,
    pub done_in: u128,
}

const CLIENTS: usize = 10000;

async fn connect_client() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, String> {
    match connect_async("ws://127.0.0.1:8080").await {
        Ok((ws, _)) => Ok(ws),
        Err(e) => Err(format!("{}", e)),
    }
}

async fn create_client(
    ws: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    status: mpsc::Sender<ClientStatus>,
    stat: Arc<RwLock<Stat>>,
    starter: mpsc::Receiver<()>,
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
                    )))
                    .expect("ClientStatus should be sent");
                return;
            }
        }
    };
    let client_status_rd = status.clone();
    let (mut write, mut read) = ws.split();
    // TODO: stream should be closed as well
    let reader = spawn(async move {
        info!(target: "test", "client: reader is created");
        if let Some(msg) = read.next().await {
            let data = msg.unwrap().into_data();
            info!(target: "test", "income data: {:?}", data);
        }
        info!(target: "test", "client: reader is destroyed");
    });
    let (tx_sender_from_client, mut rx_sender_from_client): (
        UnboundedSender<Vec<u8>>,
        UnboundedReceiver<Vec<u8>>,
    ) = unbounded_channel();
    let client_status_wr = status.clone();
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
                    )))
                    .expect("ClientStatus should be sent");
                return Err(String::from("Fail to write data"));
            }
        }
        info!(target: "test", "client: writer is destroyed");
        Ok(write)
    });
    let client_status_sd = status.clone();
    let stat_sw = stat.clone();
    let sender_from_client = spawn(async move {
        if let Err(e) = starter.recv() {
            error!(target: "test",
                "client [sender_from_client]: fail recieve from starter channel: {}",
                e
            );
            client_status_sd
                .send(ClientStatus::Err(format!(
                    "client [sender_from_client]: fail recieve from starter channel: {}",
                    e
                )))
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
                )))
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
    let (_reader, write, _sender) = join!(reader, writer, sender_from_client,);
    match match write {
        Ok(write) => write,
        Err(err) => panic!("Fail write data form client: {}", err),
    } {
        Ok(mut write) => {
            if let Err(err) = write.close().await {
                error!(target: "test", "cannot close stream: {}", err);
            }
        }
        Err(_) => {
            // Logs already done
        }
    }
    match stat.write() {
        Ok(mut stat) => stat.destroyed += 1,
        Err(_) => {
            error!(target: "test", "cannot write stat");
        }
    };
    client_status_rd
        .send(ClientStatus::Done)
        .expect("ClientStatus should be sent");
    info!(target: "test", "client: done");
}

async fn create_events_listener(
    mut rx_events: UnboundedReceiver<Events<Error>>,
    tx_server_state: mpsc::Sender<ServerState>,
    tx_sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
    tx_server_shutdown: UnboundedSender<()>,
    stat: Arc<RwLock<Stat>>,
) {
    info!(target: "test", "starting event listener");
    while let Some(event) = rx_events.recv().await {
        match event {
            Events::Ready => {
                info!(target: "test", "[T][EventsLoop] server is ready");
                if let Err(e) = tx_server_state.send(ServerState::Ready) {
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

fn connect_all() {
    info!(target: "test", "starting");
    let (tx_events, rx_events): (
        UnboundedSender<Events<Error>>,
        UnboundedReceiver<Events<Error>>,
    ) = unbounded_channel();
    let (tx_server_shutdown, mut rx_server_shutdown): (UnboundedSender<()>, UnboundedReceiver<()>) =
        unbounded_channel();
    let (tx_sender, rx_sender): (
        UnboundedSender<(Vec<u8>, Option<Uuid>)>,
        UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
    ) = unbounded_channel();
    let (tx_control, rx_control): (UnboundedSender<Control>, UnboundedReceiver<Control>) =
        unbounded_channel();
    let (tx_status, rx_status): (mpsc::Sender<ClientStatus>, mpsc::Receiver<ClientStatus>) =
        mpsc::channel();
    let (tx_server_state, rx_server_state): (
        mpsc::Sender<ServerState>,
        mpsc::Receiver<ServerState>,
    ) = mpsc::channel();
    let stat: Arc<RwLock<Stat>> = Arc::new(RwLock::new(Stat {
        created: 0,
        destroyed: 0,
        connected: 0,
        disconnected: 0,
        failed: 0,
        sent: 0,
        recieved: 0,
        created_in: 0,
        sent_in: 0,
        done_in: 0,
    }));
    thread::spawn(move || {
        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(error!(target: "test", "Fail to create runtime executor. Error: {}", e))
            }
        };
        rt.block_on(async {
            println!("{} starting server...", style("[test]").bold().dim(),);
            let mut server: Server = Server::new(String::from("127.0.0.1:8080"));
            match server.listen(tx_events, rx_sender, Some(rx_control)) {
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
        Ok(())
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
    match rx_server_state.recv() {
        Ok(state) => match state {
            ServerState::Ready => {
                println!("{} server is ready", style("[test]").bold().dim(),);
            }
        },
        Err(e) => panic!("{}", e),
    };
    let mut starters: Vec<mpsc::Sender<()>> = vec![];
    let mut connections = vec![];
    println!(
        "{} creating clients executors...",
        style("[test]").bold().dim(),
    );
    let start = Instant::now();
    for _ in 0..CLIENTS {
        let (tx_client_starter, rx_client_starter): (mpsc::Sender<()>, mpsc::Receiver<()>) =
            mpsc::channel();
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
    thread::spawn(move || {
        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(error!(target: "test", "Fail to create runtime executor. Error: {}", e))
            }
        };
        rt.block_on(async {
            println!("{} starting clients...", style("[test]").bold().dim(),);
            join_all(connections).await;
        });
        Ok(())
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
    while let Ok(status) = rx_status.recv() {
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
                // panic!("{}", e)
                match stat.write() {
                    Ok(mut stat) => stat.failed += 1,
                    Err(_) => {
                        error!(target: "test", "cannot write stat");
                    }
                };
                pb.inc(1);
                error!(target: "test", "client status error: {}", e);
            }
        }
    }
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

fn connect_one_by_one() {
    info!(target: "test", "starting");
    let (tx_events, rx_events): (
        UnboundedSender<Events<Error>>,
        UnboundedReceiver<Events<Error>>,
    ) = unbounded_channel();
    let (tx_server_shutdown, mut rx_server_shutdown): (UnboundedSender<()>, UnboundedReceiver<()>) =
        unbounded_channel();
    let (tx_sender, rx_sender): (
        UnboundedSender<(Vec<u8>, Option<Uuid>)>,
        UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
    ) = unbounded_channel();
    let (tx_control, rx_control): (UnboundedSender<Control>, UnboundedReceiver<Control>) =
        unbounded_channel();
    let (tx_status, rx_status): (mpsc::Sender<ClientStatus>, mpsc::Receiver<ClientStatus>) =
        mpsc::channel();
    let (tx_server_state, rx_server_state): (
        mpsc::Sender<ServerState>,
        mpsc::Receiver<ServerState>,
    ) = mpsc::channel();
    let stat: Arc<RwLock<Stat>> = Arc::new(RwLock::new(Stat {
        created: 0,
        destroyed: 0,
        connected: 0,
        disconnected: 0,
        failed: 0,
        sent: 0,
        recieved: 0,
        created_in: 0,
        sent_in: 0,
        done_in: 0,
    }));
    thread::spawn(move || {
        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(error!(target: "test", "Fail to create runtime executor. Error: {}", e))
            }
        };
        rt.block_on(async {
            println!("{} starting server...", style("[test]").bold().dim(),);
            let mut server: Server = Server::new(String::from("127.0.0.1:8080"));
            match server.listen(tx_events, rx_sender, Some(rx_control)) {
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
        Ok(())
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
    match rx_server_state.recv() {
        Ok(state) => match state {
            ServerState::Ready => {
                println!("{} server is ready", style("[test]").bold().dim(),);
            }
        },
        Err(e) => panic!("{}", e),
    };
    let mut connectings_tasks = vec![];
    println!(
        "{} creating clients connectors...",
        style("[test]").bold().dim(),
    );
    // let start = Instant::now();
    for _ in 0..CLIENTS {
        connectings_tasks.push(connect_client());
    }
    let stat_con = stat.clone();
    thread::spawn(move || {
        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(error!(target: "test", "Fail to create runtime executor. Error: {}", e))
            }
        };
        rt.block_on(async {
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
            let mut starters: Vec<mpsc::Sender<()>> = vec![];
            let mut connections = vec![];
            println!(
                "{} creating clients executors...",
                style("[test]").bold().dim(),
            );
            let start = Instant::now();
            let pb = ProgressBar::new(CLIENTS as u64);
            for _ in 0..sockets.len() {
                let (tx_client_starter, rx_client_starter): (mpsc::Sender<()>, mpsc::Receiver<()>) =
                    mpsc::channel();
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
        Ok(())
    });
    let start_wf = Instant::now();
    println!(
        "{} waiting for clients are doing job...",
        style("[test]").bold().dim(),
    );
    let mut done: usize = 0;
    let pb = ProgressBar::new(CLIENTS as u64);
    while let Ok(status) = rx_status.recv() {
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
                // panic!("{}", e)
                match stat.write() {
                    Ok(mut stat) => stat.failed += 1,
                    Err(_) => {
                        error!(target: "test", "cannot write stat");
                    }
                };
                pb.inc(1);
                error!(target: "test", "client status error: {}", e);
            }
        }
    }
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

fn main() {
    logs::init();
    println!(
        "{} Test #1. Connect clients all together",
        style("[test #1: start]").bold().dim(),
    );
    connect_all();
    println!("{} Done", style("[test #1: done]").bold().dim(),);
    println!(
        "{} Test #2. Connect clients one by one",
        style("[test #2: start]").bold().dim(),
    );
    connect_one_by_one();
    println!("{} Done", style("[test #2: done]").bold().dim(),);
}
