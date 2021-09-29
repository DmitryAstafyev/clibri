use super::{
    client::{Client, ClientStatus},
    config,
    stat::Stat,
};
//use console::style;
use console::style;
use fiber::{server, server::Impl};
use fiber_transport_server::{
    errors::Error,
    options::{Listener, Options},
    server::Server,
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

pub struct Test;

impl Test {
    async fn events_task(
        mut rx_server_events: UnboundedReceiver<server::Events<Error>>,
        tx_server_ready: oneshot::Sender<()>,
        tx_server_shutdown: oneshot::Sender<()>,
        tx_all_disconnected: oneshot::Sender<()>,
        tx_client_sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
        stat: Arc<RwLock<Stat>>,
    ) -> Result<(), String> {
        let mut tx_server_ready_wrapped = Some(tx_server_ready);
        let mut tx_server_shutdown_wrapped = Some(tx_server_shutdown);
        let mut disconnected: u32 = 0;
        let mut tx_all_disconnected = Some(tx_all_disconnected);
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
                server::Events::Disconnected(_uuid) => {
                    disconnected += 1;
                    match stat.write() {
                        Ok(mut stat) => stat.disconnected += 1,
                        Err(err) => {
                            return Err(format!("Fail write stat. Error: {}", err));
                        }
                    };
                    if disconnected == config::CLIENTS_1 {
                        if let Some(tx_all_disconnected) = tx_all_disconnected.take() {
                            tx_all_disconnected
                                .send(())
                                .map_err(|e| format!("{:?}", e))?;
                        }
                    }
                }
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
                    tx_client_sender
                        .send((buffer, Some(uuid)))
                        .map_err(|e| format!("{}", e))?;
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
        server.print_stat().unwrap();
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
        let tx_client_sender = server.sender();
        let tx_server_control = server.control();
        let (tx_server_ready, rx_server_ready): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        let (tx_server_shutdown, rx_server_shutdown): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        let (tx_client_status, mut rx_client_status): (
            UnboundedSender<ClientStatus>,
            UnboundedReceiver<ClientStatus>,
        ) = unbounded_channel();
        let (tx_all_disconnected, rx_all_disconnected): (
            oneshot::Sender<()>,
            oneshot::Receiver<()>,
        ) = oneshot::channel();
        let stat: Arc<RwLock<Stat>> = Arc::new(RwLock::new(Stat::new()));
        let stat_rc = stat.clone();
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
                    tx_all_disconnected,
                    tx_client_sender,
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
        let pb = ProgressBar::new(config::CLIENTS_1 as u64);
        for _ in 0..config::CLIENTS_1 {
            let client = Client::new(None)
                .await
                .map_err(|e| format!("Fail to create a client. Error: {}", e))?;
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
            let results = join_all(
                clients
                    .iter_mut()
                    .map(|(_uuid, client)| client.run(tx_client_status.clone(), stat_rc.clone())),
            )
            .await;
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
        let pb = ProgressBar::new(config::CLIENTS_1 as u64);
        while let Some(status) = rx_client_status.recv().await {
            //TODO: check state. it might be an error
            match status {
                ClientStatus::Done => {
                    done += 1;
                    pb.inc(1);
                    if done == config::CLIENTS_1 as usize {
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
        println!("{} all clients are done", style("[test]").bold().dim(),);
        match stat.write() {
            Ok(mut stat) => stat.done_in = start.elapsed().as_millis(),
            Err(err) => panic!("fail to write stat: {}", err),
        };
        // Step 6. Waiting for all clients will be disconnected
        rx_all_disconnected.await.map_err(|e| e.to_string())?;
        println!(
            "{} all clients are disconnected",
            style("[test]").bold().dim(),
        );
        // Step 7. Waiting for clients will be removed
        rx_clients_task_done.await.map_err(|e| e.to_string())?;
        println!("{} all clients are removed", style("[test]").bold().dim(),);
        // Step 8. Shutdown server
        println!(
            "{} send shutdown command to server",
            style("[test]").bold().dim(),
        );
        let (tx_sd_conf, rx_sd_conf): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        tx_server_control
            .send(server::Control::Shutdown(tx_sd_conf))
            .map_err(|e| format!("{}", e))?;
        println!(
            "{} waiting for server would be down",
            style("[test]").bold().dim(),
        );
        // Step 7. Waiting for a server is done
        if rx_sd_conf.await.is_err() {
            eprintln!("Fail to get shutdown confirmation");
        }
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
