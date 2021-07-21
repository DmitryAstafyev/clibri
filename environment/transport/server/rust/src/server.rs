use super::{
    channel::{Control, Messages},
    connection::Connection,
    errors::Error,
    handshake::Handshake as HandshakeInterface,
    stat::Stat,
};
use fiber::{
    env,
    env::logs,
    server::{
        control::Control as ServerControl, events::Events, interface::Interface, interface::Task,
    },
};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::{
    net::{TcpListener, TcpStream},
    select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    sync::oneshot::{channel, Receiver, Sender},
    task::{spawn, JoinHandle},
};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{Request, Response},
};
use uuid::Uuid;

pub struct Handshake;

impl HandshakeInterface for Handshake {}

pub struct Server {
    addr: String,
    tx_events: Option<UnboundedSender<Events<Error>>>,
    controlls: Arc<RwLock<HashMap<Uuid, UnboundedSender<Control>>>>,
    stat: Arc<RwLock<Stat>>,
}

impl Server {
    pub fn new(addr: String) -> Self {
        env::logs::init();
        Self {
            addr,
            tx_events: None,
            controlls: Arc::new(RwLock::new(HashMap::new())),
            stat: Arc::new(RwLock::new(Stat::new())),
        }
    }

    pub fn print_stat(&self) {
        if let Ok(stat) = self.stat.write() {
            stat.print();
        }
    }

    fn get_events_sender(&self) -> Result<Box<dyn Fn(Events<Error>) + Send + Sync>, Error> {
        let tx_events = if let Some(tx_events) = self.tx_events.as_ref() {
            tx_events.clone()
        } else {
            return Err(Error::Create(String::from("Fail to get Events channel")));
        };
        let stat = self.stat.clone();
        Ok(Box::new(move |event: Events<Error>| {
            match event {
                Events::Error(_, _) | Events::ConnectionError(_, _) | Events::ServerError(_) => {
                    if let Ok(mut stat) = stat.write() {
                        stat.errors();
                    }
                }
                _ => {}
            };
            if let Err(e) = tx_events.send(event) {
                warn!(
                    target: logs::targets::SERVER,
                    "Cannot send event. Error: {}", e
                );
            }
        }))
    }

    fn get_tx_events(&self) -> Result<UnboundedSender<Events<Error>>, Error> {
        if let Some(tx_events) = self.tx_events.as_ref() {
            Ok(tx_events.clone())
        } else {
            Err(Error::Create(String::from("Fail to get Events channel")))
        }
    }

    pub fn streams(
        &self,
        addr: String,
        tx_tcp_stream: UnboundedSender<TcpStream>,
        rx_shutdown: Receiver<()>,
    ) -> Result<JoinHandle<Result<(), Error>>, Error> {
        let send_event = self.get_events_sender()?;
        let stat = self.stat.clone();
        Ok(spawn(async move {
            info!(target: logs::targets::SERVER, "[task: streams]:: started");
            let listener = match TcpListener::bind(addr).await {
                Ok(listener) => listener,
                Err(e) => {
                    warn!(
                        target: logs::targets::SERVER,
                        "Fail to start server. Error: {}", e
                    );
                    send_event(Events::ServerError(Error::Create(format!("{}", e))));
                    return Err(Error::Create(format!("{}", e)));
                }
            };
            send_event(Events::Ready);
            select! {
                _ = async {
                    loop {
                        let stream = match listener.accept().await {
                            Ok((stream, _addr)) => {
                                debug!(target: logs::targets::SERVER, "Getting request to connect from: {}", _addr);
                                if let Ok(mut stat) = stat.write() { stat.connecting(); }
                                // TODO: middleware to confirm acception
                                stream
                            },
                            Err(e) => {
                                warn!(target: logs::targets::SERVER, "Cannot accept connection. Error: {}", e);
                                send_event(Events::ServerError(Error::AcceptStream(format!("{}", e))));
                                continue;
                            }
                        };
                        if let Err(e) = tx_tcp_stream.send(stream) {
                            warn!(target: logs::targets::SERVER, "Cannot share stream. Error: {}", e);
                            send_event(Events::ServerError(Error::AcceptStream(format!("{}", e))));
                            break;
                        }
                    }
                } => {
                    warn!(target: logs::targets::SERVER, "TcpListener listener task was finished by error");
                },
                _ = rx_shutdown => {
                    warn!(target: logs::targets::SERVER, "TcpListener listener task was finished by shutdown");
                }
            };
            drop(listener);
            info!(target: logs::targets::SERVER, "[task: streams]:: finished");
            Ok(())
        }))
    }

    pub fn accepting(
        &self,
        tx_messages: UnboundedSender<Messages>,
        mut rx_tcp_stream: UnboundedReceiver<TcpStream>,
        rx_shutdown: Receiver<()>,
    ) -> Result<JoinHandle<Result<(), Error>>, Error> {
        let send_event = self.get_events_sender()?;
        let tx_events = self.get_tx_events()?;
        let stat = self.stat.clone();
        let controlls = self.controlls.clone();
        Ok(spawn(async move {
            info!(target: logs::targets::SERVER, "[task: accepting]:: started");
            select! {
                _ = async {
                    while let Some(stream) = rx_tcp_stream.recv().await {
                        debug!(target: logs::targets::SERVER, "New stream has been gotten");
                        let ws = match accept_hdr_async(stream, |req: &Request, response: Response| {
                            Handshake::accept(req, response)
                        }).await {
                            Ok(ws) => ws,
                            Err(e) => {
                                warn!(target: logs::targets::SERVER, "Fail to accept stream due error: {:?}", e);
                                continue;
                            }
                        };
                        debug!(target: logs::targets::SERVER, "Connection has been accepted");
                        if let Ok(mut stat) = stat.write() { stat.connected(); }
                        let uuid = Uuid::new_v4();
                        let control = match Connection::new(uuid).attach(ws, tx_events.clone(), tx_messages.clone()).await {
                            Ok(control) => control,
                            Err(e) => {
                                warn!(target: logs::targets::SERVER, "Cannot create ws connection. Error: {}", e);
                                send_event(Events::ServerError(Error::CreateWS(format!("{}", e))));
                                continue;
                            }
                        };
                        match controlls.write() {
                            Ok(mut controlls) => {
                                controlls.entry(uuid).or_insert(control);
                                if let Ok(mut stat) = stat.write() { stat.alive(controlls.len()); }
                                debug!(target: logs::targets::SERVER, "Controll of connection has been added");
                                send_event(Events::Connected(uuid));
                            },
                            Err(e) => {
                                error!(target: logs::targets::SERVER, "Fail get controlls due error: {}", e);
                                send_event(Events::ServerError(Error::CreateWS(format!("{}", e))));
                                continue;
                            }
                        };
                    }
                } => {
                    info!(target: logs::targets::SERVER, "[task: accepting]:: natural finishing");
                },
                _ = rx_shutdown => {
                    info!(target: logs::targets::SERVER, "[task: accepting]:: shutdown called");
                }
            };
            info!(
                target: logs::targets::SERVER,
                "[task: accepting]:: finished"
            );
            Ok(())
        }))
    }

    pub fn messages(
        &self,
        mut rx_messages: UnboundedReceiver<Messages>,
        rx_shutdown: Receiver<()>,
    ) -> Result<JoinHandle<Result<(), Error>>, Error> {
        let send_event = self.get_events_sender()?;
        let stat = self.stat.clone();
        let controlls = self.controlls.clone();
        Ok(spawn(async move {
            info!(target: logs::targets::SERVER, "[task: messages]:: started");
            select! {
                _ = async {
                    while let Some(msg) = rx_messages.recv().await {
                        match msg {
                            Messages::Binary { uuid, buffer } => {
                                if let Ok(mut stat) = stat.write() { stat.recieved_bytes(buffer.len()); }
                                send_event(Events::Received(uuid, buffer))
                            },
                            Messages::Disconnect { uuid, code } => {
                                debug!(target: logs::targets::SERVER, "{}:: Client wants to disconnect (code: {:?})", uuid, code);
                                if let Ok(mut stat) = stat.write() { stat.disconnected(); }
                                match controlls.write() {
                                    Ok(mut controlls) => {
                                        if let Some(_control) = controlls.remove(&uuid) {
                                            debug!(target: logs::targets::SERVER, "{}:: Channel of connection has been removed", uuid);
                                            if let Ok(mut stat) = stat.write() { stat.alive(controlls.len()); }
                                            send_event(Events::Disconnected(uuid));
                                        } else {
                                            error!(target: logs::targets::SERVER, "{}:: Fail to find channel of connection to remove it", uuid);
                                        }
                                    },
                                    Err(e) => {
                                        error!(target: logs::targets::SERVER, "{}:: Cannot get access to controllers. Error: {}", uuid, e);
                                        send_event(Events::Error(Some(uuid), format!("{}", e)))
                                    },
                                };
                            },
                            Messages::Error { uuid, error } => send_event(Events::Error(Some(uuid), format!("{:?}", error).to_string()))
                        }
                    }
                } => {
                    info!(target: logs::targets::SERVER, "[task: messages]:: natural finishing");
                },
                _ = rx_shutdown => {
                    info!(target: logs::targets::SERVER, "[task: messages]:: shutdown called");
                }
            };
            info!(target: logs::targets::SERVER, "[task: messages]:: finished");
            Ok(())
        }))
    }

    pub fn sender(
        &self,
        mut rx_sending: UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
        rx_shutdown: Receiver<()>,
    ) -> Result<JoinHandle<Result<(), Error>>, Error> {
        let send_event = self.get_events_sender()?;
        let stat = self.stat.clone();
        let controlls = self.controlls.clone();
        Ok(spawn(async move {
            info!(target: logs::targets::SERVER, "[task: sender]:: started");
            select! {
                _ = async {
                    while let Some((buffer, uuid)) = rx_sending.recv().await {
                        match controlls.write() {
                            Ok(mut controlls) => {
                                let len = buffer.len();
                                if let Some(uuid) = uuid {
                                    if let Some(control) = controlls.get_mut(&uuid) {
                                        if let Err(e) = control.send(Control::Send(buffer)) {
                                            error!(target: logs::targets::SERVER, "{}:: Fail to close connection due error: {}", uuid, e);
                                            send_event(Events::Error(Some(uuid), format!("{}", e)))
                                        } else if let Ok(mut stat) = stat.write() { stat.sent_bytes(len); }
                                    }
                                } else {
                                    for (uuid, control) in controlls.iter_mut() {
                                        if let Err(e) = control.send(Control::Send(buffer.clone())) {
                                            error!(target: logs::targets::SERVER, "{}:: Fail to close connection due error: {}", uuid, e);
                                            send_event(Events::Error(Some(*uuid), format!("{}", e)))
                                        } else if let Ok(mut stat) = stat.write() { stat.sent_bytes(len); }
                                    }
                                }
                            },
                            Err(e) => {
                                error!(target: logs::targets::SERVER, "Cannot get access to controllers. Error: {}", e);
                                send_event(Events::Error(None, format!("{}", e)));
                                break;
                            },
                        };
                    }
                } => {
                    info!(target: logs::targets::SERVER, "[task: sender]:: natural finishing");
                },
                _ = rx_shutdown => {
                    info!(target: logs::targets::SERVER, "[task: sender]:: shutdown called");
                }
            };
            info!(target: logs::targets::SERVER, "[task: sender]:: finished");
            Ok(())
        }))
    }

    pub fn control(
        &self,
        control: Option<UnboundedReceiver<ServerControl>>,
        rx_shutdown: Receiver<()>,
    ) -> Result<JoinHandle<Result<(), Error>>, Error> {
        let send_event = self.get_events_sender()?;
        let controlls = self.controlls.clone();
        Ok(spawn(async move {
            info!(target: logs::targets::SERVER, "[task: control]:: started");
            if let Some(mut control) = control {
                select! {
                    _ = async {
                        while let Some(msg) = control.recv().await {
                            match msg {
                                ServerControl::Shutdown => {
                                    debug!(
                                        target: logs::targets::SERVER,
                                        "ServerControl::Shutdown has been called"
                                    );
                                    break;
                                }
                                ServerControl::Disconnect(uuid) => {
                                    match controlls.read() {
                                        Ok(controlls) => {
                                            if let Some(control) = controlls.get(&uuid) {
                                                if let Err(e) = control.send(Control::Disconnect) {
                                                    error!(target: logs::targets::SERVER, "{}:: Fail to send close connection command due error: {}", uuid, e);
                                                    send_event(Events::Error(Some(uuid), format!("{}", e)))
                                                }
                                            } else {
                                                error!(target: logs::targets::SERVER, "Command Disconnect has been gotten. But cannot find client: {}", uuid);
                                                send_event(Events::ServerError(Error::CreateWS(format!("Command Disconnect has been gotten. But cannot find client: {}", uuid))));
                                            }
                                        }
                                        Err(e) => {
                                            error!(
                                                target: logs::targets::SERVER,
                                                "Fail get controlls due error: {}", e
                                            );
                                            send_event(Events::ServerError(Error::CreateWS(format!(
                                                "{}",
                                                e
                                            ))));
                                        }
                                    };
                                }
                            }
                        }
                    } => {
                        info!(target: logs::targets::SERVER, "[task: control]:: natural finishing");
                    },
                    _ = rx_shutdown => {
                        info!(target: logs::targets::SERVER, "[task: control]:: shutdown called");
                    }
                };
            }
            info!(target: logs::targets::SERVER, "[task: control]:: finished");
            Ok(())
        }))
    }
}

impl Interface<Error> for Server {
    fn listen(
        &mut self,
        events: UnboundedSender<Events<Error>>,
        rx_sending: UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
        control: Option<UnboundedReceiver<ServerControl>>,
    ) -> Result<Task<Error>, Error> {
        self.tx_events = Some(events.clone());
        let addr: String = self.addr.clone();
        info!(
            target: logs::targets::SERVER,
            "[main]: will create main task"
        );
        info!(target: logs::targets::SERVER, "[main]: runtime is created");
        let (tx_tcp_stream, rx_tcp_stream): (
            UnboundedSender<TcpStream>,
            UnboundedReceiver<TcpStream>,
        ) = unbounded_channel();
        let (tx_messages, rx_messages): (UnboundedSender<Messages>, UnboundedReceiver<Messages>) =
            unbounded_channel();
        let (tx_streams_sd, rx_streams_sd): (Sender<()>, Receiver<()>) = channel();
        let (tx_accepting_sd, rx_accepting_sd): (Sender<()>, Receiver<()>) = channel();
        let (tx_messages_sd, rx_messages_sd): (Sender<()>, Receiver<()>) = channel();
        let (tx_sender_sd, rx_sender_sd): (Sender<()>, Receiver<()>) = channel();
        let (tx_control_sd, rx_control_sd): (Sender<()>, Receiver<()>) = channel();

        let streams_task = self.streams(addr, tx_tcp_stream, rx_streams_sd)?;
        let accepting_task = self.accepting(tx_messages, rx_tcp_stream, rx_accepting_sd)?;
        let messages_task = self.messages(rx_messages, rx_messages_sd)?;
        let sender_task = self.sender(rx_sending, rx_sender_sd)?;
        let control_task = self.control(control, rx_control_sd)?;

        Ok(Box::pin(async move {
            select! {
                _ = streams_task => {
                    debug!(target: logs::targets::SERVER, "[main]:: finished on streams_task");
                },
                _ = accepting_task => {
                    debug!(target: logs::targets::SERVER, "[main]:: finished on accepting_task");
                },
                _ = messages_task => {
                    debug!(target: logs::targets::SERVER, "[main]:: finished on messages_task");
                },
                _ = sender_task => {
                    debug!(target: logs::targets::SERVER, "[main]:: finished on sender_task");
                },
                _ = control_task => {
                    debug!(target: logs::targets::SERVER, "[main]:: finished on control_task");
                },
            };
            for task in (vec![
                ("streams_task", Some(tx_streams_sd)),
                ("accepting_task", Some(tx_accepting_sd)),
                ("messages_task", Some(tx_messages_sd)),
                ("sender_task", Some(tx_sender_sd)),
                ("control_task", Some(tx_control_sd)),
            ])
            .iter_mut()
            {
                if let Some(tx_shutdown) = task.1.take() {
                    if let Err(err) = tx_shutdown.send(()) {
                        warn!(
                            target: logs::targets::SERVER,
                            "Fail send finish signal to task: {}. Error: {:?}", task.0, err
                        );
                    }
                }
            }
            info!(target: logs::targets::SERVER, "[main]:: finished");
            if let Err(e) = events.send(Events::Shutdown) {
                warn!(
                    target: logs::targets::SERVER,
                    "Cannot send Events::Shutdown . Error: {}", e
                );
            }
            Ok(())
        }))
    }
}
