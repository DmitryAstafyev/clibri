#[macro_use]
extern crate lazy_static;

use fiber_transport_server::server::Server;
use std::sync::{
    Arc,
    RwLock
};
use fiber::{
    logger::{
        Logger,
        LogLevel,
    },
    server::{
        events::Events,
        interface::Interface,
        control::Control
    },
};
use tokio::{
    net::{
        TcpStream
    },
    sync::mpsc::{
        unbounded_channel,
        UnboundedReceiver,
        UnboundedSender
    },
    join,
    runtime::Runtime,
    task::{
        spawn,
        JoinHandle,
    },
    time::Duration,
};
pub use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
    WebSocketStream,
    MaybeTlsStream,
};
use uuid::Uuid;
use std::thread;
use std::sync::mpsc;
use futures::{
    StreamExt,
    SinkExt,
    executor,
    future::{join_all},
    stream::{ self, iter },
};
use std::time::Instant;
use console::style;
use indicatif::ProgressBar;

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

#[allow(non_upper_case_globals)]
pub mod tools {
    use fiber::logger::{ DefaultLogger };

    lazy_static! {
        pub static ref logger: DefaultLogger = DefaultLogger::new("Server test".to_owned(), None);
    }

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
    starter: mpsc::Receiver<()>
) {
    let ws = if let Some(ws) = ws {
        ws
    } else {
        tools::logger.verb("[T] client: starting client");
        match connect_async("ws://127.0.0.1:8080").await {
            Ok((ws, _)) => {
                tools::logger.verb("[T] handshake has been successfully completed");
                ws
            },
            Err(e) => {
                status.send(ClientStatus::Err(tools::logger.err(&format!("[T] client [connect_async]: failed to connect: {}", e)))).expect("ClientStatus should be sent");
                return;
            }
        }
    };
    let client_status_rd = status.clone();
    let (mut write, mut read) = ws.split();
    // TODO: stream should be closed as well
    let reader = spawn(async move {
        tools::logger.verb("[T] client: reader is created");
        if let Some(msg) = read.next().await {
            let data = msg.unwrap().into_data();
            tools::logger.verb(&format!("[T] income data: {:?}", data));
        }
        tools::logger.verb("[T] client: reader is destroyed");
    });
    let (tx_sender_from_client, mut rx_sender_from_client): (
        UnboundedSender<Vec<u8>>,
        UnboundedReceiver<Vec<u8>>,
    ) = unbounded_channel();
    let client_status_wr = status.clone();
    let writer: JoinHandle<Result<
        futures::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message>,
        String
    >> = spawn(async move {
        tools::logger.verb("[T] client: writer is created");
        while let Some(buffer) = rx_sender_from_client.recv().await {
            if let Err(e) = write.send(Message::Binary(buffer)).await {
                client_status_wr.send(ClientStatus::Err(tools::logger.err(&format!("[T] client [writer]: fail to send data: {}", e)))).expect("ClientStatus should be sent");
                return Err(String::from("Fail to write data"));
            }
        }
        tools::logger.verb("[T] client: writer is destroyed");
        Ok(write)
    });
    let client_status_sd = status.clone();
    let stat_sw = stat.clone();
    let sender_from_client = spawn(async move {
        if let Err(e) = starter.recv() {
            client_status_sd.send(ClientStatus::Err(tools::logger.err(&format!("[T] client [sender_from_client]: fail recieve from starter channel: {}", e)))).expect("ClientStatus should be sent");
            return;
        }
        tools::logger.verb("[T] client: sender is created");
        let buffer: Vec<u8> = vec![0u8, 1u8, 2u8, 3u8, 4u8];
        if let Err(e) = tx_sender_from_client.send(buffer) {
            client_status_sd.send(ClientStatus::Err(tools::logger.err(&format!("[T] client [tx_sender_from_client]: failed to send data: {}", e)))).expect("ClientStatus should be sent");
            return;
        } else {
            tools::logger.verb("[T] client: data has been sent");
            match stat_sw.write() {
                Ok(mut stat) => stat.sent += 1,
                Err(_) => { tools::logger.err("[T] cannot write stat"); },
            };
        }
        tools::logger.verb("[T] client: sender is destroyed");
    });
    match stat.write() {
        Ok(mut stat) => stat.created += 1,
        Err(_) => { tools::logger.err("[T] cannot write stat"); },
    };
    let (_reader, write, _sender) = join!(
        reader,
        writer,
        sender_from_client,
    );
    match match write {
        Ok(write) => write,
        Err(err) => panic!("Fail write data form client: {}", err)
    } {
        Ok(mut write) => if let Err(err) = write.close().await {
            tools::logger.err(&format!("[T] cannot close stream: {}", err));
        },
        Err(_) => {
            // Logs already done
        }
    }
    match stat.write() {
        Ok(mut stat) => stat.destroyed += 1,
        Err(_) => { tools::logger.err("[T] cannot write stat"); },
    };
    client_status_rd.send(ClientStatus::Done).expect("ClientStatus should be sent");
    tools::logger.verb("[T] client: done");
}

async fn create_events_listener(
    mut rx_events: UnboundedReceiver<Events>,
    tx_server_state: mpsc::Sender<ServerState>,
    tx_sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
    tx_server_shutdown: UnboundedSender<()>,
    stat: Arc<RwLock<Stat>>
) {
    tools::logger.verb("[T] starting event listener");
    while let Some(event) = rx_events.recv().await {
        match event {
            Events::Ready => {
                tools::logger.verb("[T][EventsLoop] server is ready");
                if let Err(e) = tx_server_state.send(ServerState::Ready) {
                    tools::logger.err(&format!("[T] cannot send server state: {}", e));
                }
            },
            Events::Shutdown => {
                if let Err(e) = tx_server_shutdown.send(()) {
                    tools::logger.err(&format!("[T] cannot send server state: {}", e));
                }
            },
            Events::Connected(uuid) => {
                tools::logger.verb(&format!("[T][EventsLoop] {} connected", uuid.clone()));
                match stat.write() {
                    Ok(mut stat) => stat.connected += 1,
                    Err(_) => { tools::logger.err("[T] cannot write stat"); },
                };
            },
            Events::Disconnected(uuid) => {
                tools::logger.verb(&format!("[T][EventsLoop] {} disconnected", uuid));
                match stat.write() {
                    Ok(mut stat) => stat.disconnected += 1,
                    Err(_) => { tools::logger.err("[T] cannot write stat"); },
                };
            },
            Events::Received(uuid, buffer) => {
                tools::logger.verb(&format!("[T][EventsLoop] {} data has been received: {:?}", uuid, buffer));
                match stat.write() {
                    Ok(mut stat) => stat.recieved += 1,
                    Err(_) => { tools::logger.err("[T] cannot write stat"); },
                };
                let buffer: Vec<u8> = vec![5u8, 6u8, 7u8, 8u8, 9u8];
                if let Err(e) = tx_sender.send((buffer, Some(uuid))) {
                    tools::logger.err(&format!("[T] fail to send data to connection {}: {}", uuid, e));
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
}

fn connect_all() {
    tools::logger.verb("[T] starting");
    let (tx_events, rx_events): (
        UnboundedSender<Events>,
        UnboundedReceiver<Events>,
    ) = unbounded_channel();
    let (tx_server_shutdown, mut rx_server_shutdown): (
        UnboundedSender<()>,
        UnboundedReceiver<()>,
    ) = unbounded_channel();
    let (tx_sender, rx_sender): (
        UnboundedSender<(Vec<u8>, Option<Uuid>)>,
        UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
    ) = unbounded_channel();
    let (tx_control, rx_control): (
        UnboundedSender<Control>,
        UnboundedReceiver<Control>,
    ) = unbounded_channel();
    let (tx_status, rx_status): (
        mpsc::Sender<ClientStatus>,
        mpsc::Receiver<ClientStatus>,
    ) = mpsc::channel();
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
        let rt  = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(tools::logger.err(&format!("Fail to create runtime executor. Error: {}", e)))
            },
        };
        rt.block_on(async {
            println!(
                "{} starting server...",
                style("[test]").bold().dim(),
            );
            let mut server: Server = Server::new(String::from("127.0.0.1:8080"));
            if let Err(e) = server.listen(tx_events, rx_sender, Some(rx_control)).await {
                tools::logger.err(&format!("[T] fail to create server: {}", e));
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
            stat_sr
        ));
    });
    println!(
        "{} waiting for server...",
        style("[test]").bold().dim(),
    );
    match rx_server_state.recv() {
        Ok(state) => match state {
            ServerState::Ready => {
                println!(
                    "{} server is ready",
                    style("[test]").bold().dim(),
                );
            },
        },
        Err(e) => panic!(e)
    };
    let mut starters: Vec<mpsc::Sender<()>> = vec![];
    let mut connections = vec![];
    println!(
        "{} creating clients executors...",
        style("[test]").bold().dim(),
    );
    let start = Instant::now();
    for _ in 0..CLIENTS {
        let (tx_client_starter, rx_client_starter): (
            mpsc::Sender<()>,
            mpsc::Receiver<()>,
        ) = mpsc::channel();
        let client = create_client(
            None,
            tx_status.clone(),
            stat.clone(),
            rx_client_starter,
        );
        connections.push(client);
        starters.push(tx_client_starter);
    }
    match stat.write() {
        Ok(mut stat) => stat.created_in = start.elapsed().as_millis(),
        Err(_) => { tools::logger.err("[T] cannot write stat"); },
    };
    thread::spawn(move || {
        let rt  = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(tools::logger.err(&format!("Fail to create runtime executor. Error: {}", e)))
            },
        };
        rt.block_on(async {
            println!(
                "{} starting clients...",
                style("[test]").bold().dim(),
            );
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
        Err(_) => { tools::logger.err("[T] cannot write stat"); },
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
            },
            ClientStatus::Err(e) => {
                // panic!(e)
                match stat.write() {
                    Ok(mut stat) => stat.failed += 1,
                    Err(_) => { tools::logger.err("[T] cannot write stat"); },
                };
                pb.inc(1);
                tools::logger.err(&format!("[T] client status error: {}", e));
            }
        } 
        
    }
    pb.finish_and_clear();
    match stat.write() {
        Ok(mut stat) => stat.done_in = start_wf.elapsed().as_millis(),
        Err(_) => { tools::logger.err("[T] cannot write stat"); },
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
            tools::logger.err(&format!("Fail send Control::Shutdown. Error: {}", e));
        }
        println!(
            "{} waiting for shutdown server...",
            style("[test]").bold().dim(),
        );
        rx_server_shutdown.recv().await;
        println!(
            "{} server is shutdown",
            style("[test]").bold().dim(),
        );
    });
}

fn connect_one_by_one() {
    tools::logger.verb("[T] starting");
    let (tx_events, rx_events): (
        UnboundedSender<Events>,
        UnboundedReceiver<Events>,
    ) = unbounded_channel();
    let (tx_server_shutdown, mut rx_server_shutdown): (
        UnboundedSender<()>,
        UnboundedReceiver<()>,
    ) = unbounded_channel();
    let (tx_sender, rx_sender): (
        UnboundedSender<(Vec<u8>, Option<Uuid>)>,
        UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
    ) = unbounded_channel();
    let (tx_control, rx_control): (
        UnboundedSender<Control>,
        UnboundedReceiver<Control>,
    ) = unbounded_channel();
    let (tx_status, rx_status): (
        mpsc::Sender<ClientStatus>,
        mpsc::Receiver<ClientStatus>,
    ) = mpsc::channel();
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
        let rt  = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(tools::logger.err(&format!("Fail to create runtime executor. Error: {}", e)))
            },
        };
        rt.block_on(async {
            println!(
                "{} starting server...",
                style("[test]").bold().dim(),
            );
            let mut server: Server = Server::new(String::from("127.0.0.1:8080"));
            if let Err(e) = server.listen(tx_events, rx_sender, Some(rx_control)).await {
                tools::logger.err(&format!("[T] fail to create server: {}", e));
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
            stat_sr
        ));
    });
    println!(
        "{} waiting for server...",
        style("[test]").bold().dim(),
    );
    match rx_server_state.recv() {
        Ok(state) => match state {
            ServerState::Ready => {
                println!(
                    "{} server is ready",
                    style("[test]").bold().dim(),
                );
            },
        },
        Err(e) => panic!(e)
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
    // match stat.write() {
    //     Ok(mut stat) => stat.created_in = start.elapsed().as_millis(),
    //     Err(_) => { tools::logger.err("[T] cannot write stat"); },
    // };
    let stat_con = stat.clone();
    thread::spawn(move || {
        let rt  = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(tools::logger.err(&format!("Fail to create runtime executor. Error: {}", e)))
            },
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
                    },
                    Err(err) => panic!(err)
                };
            };
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
                let (tx_client_starter, rx_client_starter): (
                    mpsc::Sender<()>,
                    mpsc::Receiver<()>,
                ) = mpsc::channel();
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
                Err(_) => { tools::logger.err("[T] cannot write stat"); },
            };
            println!(
                "{} starting clients...",
                style("[test]").bold().dim(),
            );
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
                Err(_) => { tools::logger.err("[T] cannot write stat"); },
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
            },
            ClientStatus::Err(e) => {
                // panic!(e)
                match stat.write() {
                    Ok(mut stat) => stat.failed += 1,
                    Err(_) => { tools::logger.err("[T] cannot write stat"); },
                };
                pb.inc(1);
                tools::logger.err(&format!("[T] client status error: {}", e));
            }
        } 
        
    }
    pb.finish_and_clear();
    match stat.write() {
        Ok(mut stat) => stat.done_in = start_wf.elapsed().as_millis(),
        Err(_) => { tools::logger.err("[T] cannot write stat"); },
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
            tools::logger.err(&format!("Fail send Control::Shutdown. Error: {}", e));
        }
        println!(
            "{} waiting for shutdown server...",
            style("[test]").bold().dim(),
        );
        rx_server_shutdown.recv().await;
        println!(
            "{} server is shutdown",
            style("[test]").bold().dim(),
        );
    });
}

fn main() {
    match fiber::tools::LOGGER_SETTINGS.lock() {
        Ok(mut settings) => settings.set_level(LogLevel::Error),
        Err(e) => println!("Fail set log level due error: {}", e),
    };
    println!(
        "{} Test #1. Connect clients all together",
        style("[test #1: start]").bold().dim(),
    );
    connect_all();
    println!(
        "{} Done",
        style("[test #1: done]").bold().dim(),
    );
    println!(
        "{} Test #2. Connect clients one by one",
        style("[test #2: start]").bold().dim(),
    );
    connect_one_by_one();
    println!(
        "{} Done",
        style("[test #2: done]").bold().dim(),
    );
}
