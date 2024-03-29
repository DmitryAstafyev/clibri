use super::{client::ClientStatus, config, stat::Stat};
use console::style;
use clibri::{
    server,
    server::{Control as ControlTrait, Impl},
};
use clibri_transport_client::{
    client::{Client, ConnectReturn, ToSend},
    events::{Event as ClientEvent, Message as ClientMessage},
    options::{ConnectionType, Options as ClientOptions},
};
use clibri_transport_server::{
    errors::Error,
    options::{Listener, Options},
    server::{Control, Server},
};
use futures::future::join_all;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use tokio::{
    select,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task::spawn,
};
use uuid::Uuid;

// TODO: clients events: connected / disconnected don't work
pub struct Test;

impl Test {
    async fn events_task(
        mut rx_server_events: UnboundedReceiver<server::Events<Error>>,
        tx_server_ready: oneshot::Sender<()>,
        tx_server_shutdown: oneshot::Sender<()>,
        server_control: Control,
        stat: Arc<RwLock<Stat>>,
    ) -> Result<(), String> {
        let mut tx_server_ready_wrapped = Some(tx_server_ready);
        let mut tx_server_shutdown_wrapped = Some(tx_server_shutdown);
        while let Some(event) = rx_server_events.recv().await {
            match event {
                server::Events::Ready => {
                    if let Some(tx_server_ready) = tx_server_ready_wrapped.take() {
                        tx_server_ready.send(()).map_err(|e| format!("{:?}", e))?;
                    } else {
                        return Err(String::from("Server Ready event called twice"));
                    }
                }
                server::Events::Shutdown => {
                    if let Some(tx_server_shutdown) = tx_server_shutdown_wrapped.take() {
                        tx_server_shutdown
                            .send(())
                            .map_err(|e| format!("{:?}", e))?;
                    } else {
                        return Err(String::from("Server Shutdown event called twice"));
                    }
                }
                server::Events::Connected(_uuid) => {
                    match stat.write() {
                        Ok(mut stat) => stat.connected += 1,
                        Err(err) => {
                            return Err(format!("Fail write stat. Error: {}", err));
                        }
                    };
                }
                server::Events::Disconnected(_uuid) => {}
                server::Events::Received(uuid, buffer) => {
                    match stat.write() {
                        Ok(mut stat) => stat.recieved += 1,
                        Err(err) => {
                            return Err(format!("Fail write stat. Error: {}", err));
                        }
                    };
                    if buffer != vec![0u8, 1u8, 2u8, 3u8, 4u8] {
                        eprintln!("Invalid data come to server from client");
                    }
                    let buffer: Vec<u8> = vec![5u8, 6u8, 7u8, 8u8, 9u8];
                    server_control
                        .send(buffer, Some(uuid))
                        .await
                        .map_err(|e| e.to_string())?;
                    match stat.write() {
                        Ok(mut stat) => stat.sent += 1,
                        Err(err) => {
                            return Err(format!("Fail write stat. Error: {}", err));
                        }
                    };
                }
                server::Events::Error(uuid, err) => {
                    return Err(format!("Error ({:?}): {:?}", uuid, err));
                }
                server::Events::ConnectionError(uuid, err) => {
                    return Err(format!("ConnectionError ({:?}): {:?}", uuid, err));
                }
                server::Events::ServerError(err) => {
                    return Err(format!("ServerError: {:?}", err));
                }
            }
        }
        Ok(())
    }

    async fn server_task(mut server: Server) -> Result<(), String> {
        let result = server.listen().await;
        result.map_err(|e| format!("{}", e))
    }

    pub async fn run() -> Result<(), String> {
        let socket_addr = config::SERVER_ADDR
            .parse::<SocketAddr>()
            .map_err(|e| e.to_string())?;
        let mut server: Server = Server::new(Options {
            listener: Listener::Direct(socket_addr),
        });
        let rx_server_events = server.observer().map_err(|e| format!("{:?}", e))?;
        let server_control = server.control();
        let (tx_server_ready, rx_server_ready): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        let (tx_server_shutdown, rx_server_shutdown): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        let (tx_client_status, mut rx_client_status): (
            UnboundedSender<ClientStatus>,
            UnboundedReceiver<ClientStatus>,
        ) = unbounded_channel();
        let stat: Arc<RwLock<Stat>> = Arc::new(RwLock::new(Stat::new()));
        let stat_rc = stat.clone();
        let server_control_cl = server_control.clone();
        // Step 1. Start server and server events loop
        spawn(async move {
            println!(
                "{} spawning server and server events loop",
                style("[test]").bold().dim(),
            );
            if let Err(err) = select! {
                res = Self::server_task(server) => res,
                res = Self::events_task(
                    rx_server_events,
                    tx_server_ready,
                    tx_server_shutdown,
                    server_control_cl,
                    stat_rc
                ) => res
            } {
                panic!("{:?}", err);
            }
            println!(
                "{} server and server events loop are closed",
                style("[test]").bold().dim(),
            );
        });
        // Step 2. Waiting for a server or error
        println!("{} waiting for server", style("[test]").bold().dim(),);
        rx_server_ready.await.map_err(|e| format!("{}", e))?;
        // Step 3. Create clients
        println!("{} creating clients", style("[test]").bold().dim(),);
        let start = Instant::now();
        let mut clients: HashMap<Uuid, Client> = HashMap::new();
        let pb = ProgressBar::new(config::CLIENTS_2 as u64);
        let socket_addr = config::SERVER_ADDR
            .parse::<SocketAddr>()
            .map_err(|e| e.to_string())?;
        for _ in 0..config::CLIENTS_4 {
            let client = Client::new(
                ClientOptions {
                    connection: ConnectionType::Direct(socket_addr),
                },
                None,
            );
            let uuid = client.uuid();
            clients.insert(uuid, client);
            match stat.write() {
                Ok(mut stat) => stat.created += 1,
                Err(err) => {
                    return Err(format!("Fail write stat. Error: {}", err));
                }
            };
            pb.inc(1);
        }
        pb.finish_and_clear();
        println!("{} clients are created", style("[test]").bold().dim(),);
        match stat.write() {
            Ok(mut stat) => stat.created_in = start.elapsed().as_millis(),
            Err(err) => panic!("fail to write stat: {}", err),
        };
        // Step 4. Start clients
        let (tx_clients_task_done, rx_clients_task_done): (
            oneshot::Sender<()>,
            oneshot::Receiver<()>,
        ) = oneshot::channel();
        let stat_rc = stat.clone();
        spawn(async move {
            println!(
                "{} clients spawn task: started",
                style("[test]").bold().dim(),
            );
            let start = Instant::now();
            let mut jobs = vec![];
            println!(
                "{} clients spawn task: creating clients and jobs",
                style("[test]").bold().dim(),
            );
            let pb = ProgressBar::new(config::CLIENTS_2 as u64);
            for (_, client) in clients.iter_mut() {
                let (mut rx_client_event, rx_client_done): ConnectReturn =
                    match client.connect().await {
                        Ok(connection) => connection,
                        Err(err) => {
                            eprintln!("fail to connect client: {:?}", err);
                            panic!("fail to connect client: {:?}", err);
                        }
                    };
                let stat = stat_rc.clone();
                let client_ref = client.clone();
                let status = tx_client_status.clone();
                jobs.push(async move {
                    // Step 1. Wakeup
                    match stat.write() {
                        Ok(mut stat) => stat.wakeup += 1,
                        Err(err) => {
                            return Err(format!("Fail write stat. Error: {}", err));
                        }
                    };
                    // Step 2. Sending sample package to server
                    let buffer: Vec<u8> = vec![0u8, 1u8, 2u8, 3u8, 4u8];
                    client_ref
                        .send(ToSend::Binary(buffer.clone()))
                        .map_err(|e| format!("{:?}", e))?;
                    match stat.write() {
                        Ok(mut stat) => stat.write += 1,
                        Err(err) => {
                            return Err(format!("Fail write stat. Error: {}", err));
                        }
                    };
                    let mut flag_sent: bool = false;
                    let mut flag_connected: bool = false;
                    while let Some(msg) = rx_client_event.recv().await {
                        match msg {
                            // Step 3. Waiting and reading message from server
                            ClientEvent::Message(msg) => match msg {
                                ClientMessage::Binary(income) => {
                                    if flag_sent {
                                        return Err(String::from("Binary event triggered twice"));
                                    }
                                    flag_sent = true;
                                    if income != vec![5u8, 6u8, 7u8, 8u8, 9u8] {
                                        return Err(String::from("Invalid data from server"));
                                    }
                                    match stat.write() {
                                        Ok(mut stat) => stat.read += 1,
                                        Err(err) => {
                                            return Err(format!("Fail write stat. Error: {}", err));
                                        }
                                    };
                                    match stat.write() {
                                        Ok(mut stat) => stat.client_done += 1,
                                        Err(err) => {
                                            return Err(format!("Fail write stat. Error: {}", err));
                                        }
                                    };
                                    status.send(ClientStatus::Done).map_err(|e| e.to_string())?;
                                }
                                smth => {
                                    eprintln!("unexpected message: {:?}", smth);
                                    return Err(format!("unexpected message: {:?}", smth));
                                }
                            },
                            ClientEvent::Connected(_) => {
                                if flag_connected {
                                    return Err(String::from("Connected event triggered twice"));
                                }
                                flag_connected = true;
                                match stat.write() {
                                    Ok(mut stat) => stat.client_connected += 1,
                                    Err(err) => {
                                        return Err(format!("Fail write stat. Error: {}", err));
                                    }
                                };
                            }
                            ClientEvent::Disconnected => {
                                match stat.write() {
                                    Ok(mut stat) => stat.disconnected += 1,
                                    Err(err) => {
                                        return Err(format!("Fail write stat. Error: {}", err));
                                    }
                                };
                            }
                            ClientEvent::Error(err) => {
                                eprintln!("client error: {:?}", err);
                                return Err(format!("client error: {:?}", err));
                            }
                        }
                    }
                    if rx_client_done.await.is_err() {
                        eprintln!("fail to get done signal from client");
                        return Err(String::from("fail to get done signal from client"));
                    }
                    Ok::<Uuid, String>(client.uuid())
                });
                pb.inc(1);
            }
            pb.finish_and_clear();
            println!(
                "{} clients spawn task: waiting for clients will do jobs",
                style("[test]").bold().dim(),
            );
            let results = join_all(jobs).await;
            println!(
                "{} clients spawn task: checks for clients jobs results",
                style("[test]").bold().dim(),
            );
            for result in results.iter() {
                match result {
                    Ok(uuid) => {
                        if clients.remove(&uuid).is_some() {
                            match stat_rc.write() {
                                Ok(mut stat) => stat.destroyed += 1,
                                Err(err) => panic!("fail to write stat: {}", err),
                            };
                        }
                    }
                    Err(err) => panic!("{}", err),
                };
            }
            match stat_rc.write() {
                Ok(mut stat) => stat.created_in = start.elapsed().as_millis(),
                Err(err) => panic!("fail to write stat: {}", err),
            };
            println!("{} clients spawn task: done", style("[test]").bold().dim(),);
            drop(clients);
            if let Err(err) = tx_clients_task_done.send(()) {
                panic!("{:?}", err);
            }
        });
        // Step 5. Waiting for a clients
        println!(
            "{} waiting for clients are doing a job",
            style("[test]").bold().dim(),
        );
        let mut done: usize = 0;
        let start = Instant::now();
        let pb = ProgressBar::new(config::CLIENTS_2 as u64);
        while let Some(status) = rx_client_status.recv().await {
            //TODO: check state. it might be an error
            match status {
                ClientStatus::Done => {
                    done += 1;
                    pb.inc(1);
                    if done == config::CLIENTS_2 as usize {
                        break;
                    }
                }
                ClientStatus::Err(e) => {
                    eprintln!("client status error: {}", e);
                    match stat.write() {
                        Ok(mut stat) => stat.failed += 1,
                        Err(err) => panic!("fail to write stat: {}", err),
                    };
                    pb.inc(1);
                }
            }
        }
        pb.finish_and_clear();
        match stat.write() {
            Ok(mut stat) => stat.done_in = start.elapsed().as_millis(),
            Err(err) => panic!("fail to write stat: {}", err),
        };
        // Step 8. Shutdown server
        println!(
            "{} send shutdown command to server",
            style("[test]").bold().dim(),
        );
        server_control.shutdown().await.map_err(|e| e.to_string())?;
        // Step 7. Waiting for clients will be removed
        rx_clients_task_done.await.map_err(|e| e.to_string())?;
        println!("{} all clients are removed", style("[test]").bold().dim(),);

        // server_control
        //     .print_stat()
        //     .await
        //     .map_err(|e| e.to_string())?;
        // server_control.shutdown().await.map_err(|e| e.to_string())?;
        // Step 7. Waiting for a server is done

        //rx_server_shutdown.await.map_err(|e| format!("{}", e))?;
        if let Err(err) = rx_server_shutdown.await {
            if err.to_string() != *"channel closed" {
                return Err(err.to_string());
            }
        }
        match stat.read() {
            Ok(stat) => stat.print(),
            Err(err) => panic!("fail to read stat: {}", err),
        };
        Ok(())
    }
}
