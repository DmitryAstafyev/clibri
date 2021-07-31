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
        control::Control as ServerControl,
        events::Events,
        interface::{Interface, Sending, Task},
    },
};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::{
    net::{TcpListener, TcpStream},
    select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::{spawn, JoinHandle},
};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{Request, Response},
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub struct Handshake;

impl HandshakeInterface for Handshake {}

pub struct Server {
    addr: String,
    tx_events: UnboundedSender<Events<Error>>,
    rx_events: Option<UnboundedReceiver<Events<Error>>>,
    tx_sender: UnboundedSender<Sending>,
    rx_sender: Option<UnboundedReceiver<Sending>>,
    tx_control: UnboundedSender<ServerControl>,
    rx_control: Option<UnboundedReceiver<ServerControl>>,
    controlls: Arc<RwLock<HashMap<Uuid, UnboundedSender<Control>>>>,
    stat: Arc<RwLock<Stat>>,
}

impl Server {
    pub fn new(addr: String) -> Self {
        env::logs::init();
        let (tx_events, rx_events): (
            UnboundedSender<Events<Error>>,
            UnboundedReceiver<Events<Error>>,
        ) = unbounded_channel();
        let (tx_sender, rx_sender): (UnboundedSender<Sending>, UnboundedReceiver<Sending>) =
            unbounded_channel();
        let (tx_control, rx_control): (
            UnboundedSender<ServerControl>,
            UnboundedReceiver<ServerControl>,
        ) = unbounded_channel();
        Self {
            addr,
            tx_events,
            rx_events: Some(rx_events),
            tx_sender,
            rx_sender: Some(rx_sender),
            tx_control,
            rx_control: Some(rx_control),
            controlls: Arc::new(RwLock::new(HashMap::new())),
            stat: Arc::new(RwLock::new(Stat::new())),
        }
    }

    pub fn print_stat(&self) {
        if let Ok(stat) = self.stat.write() {
            stat.print();
        }
    }

    fn streams_task(
        &self,
        addr: String,
        tx_tcp_stream: UnboundedSender<TcpStream>,
        cancel: CancellationToken,
    ) -> Result<JoinHandle<Result<(), Error>>, Error> {
        let tx_events = self.tx_events.clone();
        let stat = self.stat.clone();
        Ok(spawn(async move {
            info!(target: logs::targets::SERVER, "[task: streams]:: started");
            let listener = match TcpListener::bind(addr).await {
                Ok(listener) => listener,
                Err(e) => {
                    error!(
                        target: logs::targets::SERVER,
                        "Fail to start server. Error: {}", e
                    );
                    tx_events
                        .send(Events::ServerError(Error::Create(format!("{}", e))))
                        .map_err(|e| Error::Channel(e.to_string()))?;
                    return Err(Error::Create(format!("{}", e)));
                }
            };
            tx_events
                .send(Events::Ready)
                .map_err(|e| Error::Channel(e.to_string()))?;
            let res = select! {
                res = async {
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
                                tx_events.send(Events::ServerError(Error::AcceptStream(format!("{}", e)))).map_err(|e| Error::Channel(e.to_string()))?;
                                continue;
                            }
                        };
                        if let Err(e) = tx_tcp_stream.send(stream) {
                            warn!(target: logs::targets::SERVER, "Cannot share stream. Error: {}", e);
                            tx_events.send(Events::ServerError(Error::AcceptStream(format!("{}", e)))).map_err(|e| Error::Channel(e.to_string()))?;
                            return Err(Error::AcceptStream(format!("{}", e)));
                        }
                    }
                } => res,
                _ = cancel.cancelled() => {
                    debug!(target: logs::targets::SERVER, "TcpListener listener task was finished by shutdown");
                    Ok(())
                }
            };
            drop(listener);
            res
        }))
    }

    fn accepting_task(
        &self,
        tx_messages: UnboundedSender<Messages>,
        mut rx_tcp_stream: UnboundedReceiver<TcpStream>,
        cancel: CancellationToken,
    ) -> Result<JoinHandle<Result<(), Error>>, Error> {
        let tx_events = self.tx_events.clone();
        let stat = self.stat.clone();
        let controlls = self.controlls.clone();
        Ok(spawn(async move {
            info!(target: logs::targets::SERVER, "[task: accepting]:: started");
            select! {
                res = async {
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
                                tx_events.send(Events::ServerError(Error::CreateWS(format!("{}", e)))).map_err(|e| Error::Channel(e.to_string()))?;
                                continue;
                            }
                        };
                        match controlls.write() {
                            Ok(mut controlls) => {
                                controlls.entry(uuid).or_insert(control);
                                if let Ok(mut stat) = stat.write() { stat.alive(controlls.len()); }
                                debug!(target: logs::targets::SERVER, "Controll of connection has been added");
                                tx_events.send(Events::Connected(uuid)).map_err(|e| Error::Channel(e.to_string()))?;
                            },
                            Err(e) => {
                                error!(target: logs::targets::SERVER, "Fail get controlls due error: {}", e);
                                tx_events.send(Events::ServerError(Error::CreateWS(format!("{}", e)))).map_err(|e| Error::Channel(e.to_string()))?;
                                continue;
                            }
                        };
                    }
                    Ok(())
                } => res,
                _ = cancel.cancelled() => {
                    info!(target: logs::targets::SERVER, "[task: accepting]:: shutdown called");
                    Ok(())
                }
            }
        }))
    }

    fn messages_task(
        &self,
        mut rx_messages: UnboundedReceiver<Messages>,
        cancel: CancellationToken,
    ) -> Result<JoinHandle<Result<(), Error>>, Error> {
        let tx_events = self.tx_events.clone();
        let stat = self.stat.clone();
        let controlls = self.controlls.clone();
        Ok(spawn(async move {
            info!(target: logs::targets::SERVER, "[task: messages]:: started");
            select! {
                res = async {
                    while let Some(msg) = rx_messages.recv().await {
                        match msg {
                            Messages::Binary { uuid, buffer } => {
                                if let Ok(mut stat) = stat.write() { stat.recieved_bytes(buffer.len()); }
                                tx_events.send(Events::Received(uuid, buffer)).map_err(|e| Error::Channel(e.to_string()))?;
                            },
                            Messages::Disconnect { uuid, code } => {
                                debug!(target: logs::targets::SERVER, "{}:: Client wants to disconnect (code: {:?})", uuid, code);
                                if let Ok(mut stat) = stat.write() { stat.disconnected(); }
                                match controlls.write() {
                                    Ok(mut controlls) => {
                                        if let Some(_control) = controlls.remove(&uuid) {
                                            debug!(target: logs::targets::SERVER, "{}:: Channel of connection has been removed", uuid);
                                            if let Ok(mut stat) = stat.write() { stat.alive(controlls.len()); }
                                            tx_events.send(Events::Disconnected(uuid)).map_err(|e| Error::Channel(e.to_string()))?;
                                        } else {
                                            error!(target: logs::targets::SERVER, "{}:: Fail to find channel of connection to remove it", uuid);
                                        }
                                    },
                                    Err(e) => {
                                        error!(target: logs::targets::SERVER, "{}:: Cannot get access to controllers. Error: {}", uuid, e);
                                        tx_events.send(Events::Error(Some(uuid), format!("{}", e))).map_err(|e| Error::Channel(e.to_string()))?;
                                    },
                                };
                            },
                            Messages::Error { uuid, error } => {
                                tx_events.send(Events::Error(Some(uuid), format!("{:?}", error).to_string())).map_err(|e| Error::Channel(e.to_string()))?;
                            }
                        }
                    }
                    Ok(())
                } => res,
                _ = cancel.cancelled() => {
                    info!(target: logs::targets::SERVER, "[task: messages]:: shutdown called");
                    Ok(())
                }
            }
        }))
    }

    fn sending_task(
        &self,
        mut rx_sending: UnboundedReceiver<Sending>,
        cancel: CancellationToken,
    ) -> Result<JoinHandle<Result<(), Error>>, Error> {
        let tx_events = self.tx_events.clone();
        let stat = self.stat.clone();
        let controlls = self.controlls.clone();
        Ok(spawn(async move {
            info!(target: logs::targets::SERVER, "[task: sender]:: started");
            select! {
                res = async {
                    while let Some((buffer, uuid)) = rx_sending.recv().await {
                        match controlls.write() {
                            Ok(mut controlls) => {
                                let len = buffer.len();
                                if let Some(uuid) = uuid {
                                    if let Some(control) = controlls.get_mut(&uuid) {
                                        if let Err(e) = control.send(Control::Send(buffer)) {
                                            error!(target: logs::targets::SERVER, "{}:: Fail to close connection due error: {}", uuid, e);
                                            tx_events.send(Events::Error(Some(uuid), format!("{}", e))).map_err(|e| Error::Channel(e.to_string()))?;
                                        } else if let Ok(mut stat) = stat.write() { stat.sent_bytes(len); }
                                    }
                                } else {
                                    for (uuid, control) in controlls.iter_mut() {
                                        if let Err(e) = control.send(Control::Send(buffer.clone())) {
                                            error!(target: logs::targets::SERVER, "{}:: Fail to close connection due error: {}", uuid, e);
                                            tx_events.send(Events::Error(Some(*uuid), format!("{}", e))).map_err(|e| Error::Channel(e.to_string()))?;
                                        } else if let Ok(mut stat) = stat.write() { stat.sent_bytes(len); }
                                    }
                                }
                            },
                            Err(e) => {
                                error!(target: logs::targets::SERVER, "Cannot get access to controllers. Error: {}", e);
                                tx_events.send(Events::Error(None, format!("{}", e))).map_err(|e| Error::Channel(e.to_string()))?;
                                break;
                            },
                        };
                    }
                    Ok(())
                } => res,
                _ = cancel.cancelled() => {
                    info!(target: logs::targets::SERVER, "[task: sender]:: shutdown called");
                    Ok(())
                }
            }
        }))
    }

    pub fn control_task(
        &self,
        mut control: UnboundedReceiver<ServerControl>,
        cancel: CancellationToken,
    ) -> Result<JoinHandle<Result<(), Error>>, Error> {
        let tx_events = self.tx_events.clone();
        let controlls = self.controlls.clone();
        Ok(spawn(async move {
            info!(target: logs::targets::SERVER, "[task: control]:: started");
            select! {
                res = async {
                    while let Some(msg) = control.recv().await {
                        match msg {
                            ServerControl::Shutdown => {
                                debug!(
                                    target: logs::targets::SERVER,
                                    "ServerControl::Shutdown has been called"
                                );
                                return Ok(());
                            }
                            ServerControl::Disconnect(uuid) => {
                                match controlls.read() {
                                    Ok(controlls) => {
                                        if let Some(control) = controlls.get(&uuid) {
                                            if let Err(e) = control.send(Control::Disconnect) {
                                                error!(target: logs::targets::SERVER, "{}:: Fail to send close connection command due error: {}", uuid, e);
                                                tx_events.send(Events::Error(Some(uuid), format!("{}", e))).map_err(|e| Error::Channel(e.to_string()))?;
                                            }
                                        } else {
                                            error!(target: logs::targets::SERVER, "Command Disconnect has been gotten. But cannot find client: {}", uuid);
                                            tx_events.send(Events::ServerError(Error::CreateWS(format!("Command Disconnect has been gotten. But cannot find client: {}", uuid)))).map_err(|e| Error::Channel(e.to_string()))?;
                                        }
                                    }
                                    Err(e) => {
                                        error!(
                                            target: logs::targets::SERVER,
                                            "Fail get controlls due error: {}", e
                                        );
                                        tx_events.send(Events::ServerError(Error::CreateWS(format!(
                                            "{}",
                                            e
                                        )))).map_err(|e| Error::Channel(e.to_string()))?;
                                    }
                                };
                            }
                        }
                    }
                    Ok(())
                } => {
                    cancel.cancel();
                    res
                },
                _ = cancel.cancelled() => {
                    info!(target: logs::targets::SERVER, "[task: control]:: shutdown called");
                    Ok(())
                }
            }
        }))
    }
}

impl Interface<Error> for Server {
    fn listen(&mut self) -> Result<Task<Error>, Error> {
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
        let cancel: CancellationToken = CancellationToken::new();
        let rx_sender = if let Some(rx_sender) = self.rx_sender.take() {
            rx_sender
        } else {
            return Err(Error::FailTakeSender);
        };
        let rx_control = if let Some(rx_control) = self.rx_control.take() {
            rx_control
        } else {
            return Err(Error::FailTakeControl);
        };
        let streams_task = self.streams_task(addr, tx_tcp_stream, cancel.child_token())?;
        let accepting_task = self.accepting_task(tx_messages, rx_tcp_stream, cancel.child_token())?;
        let messages_task = self.messages_task(rx_messages, cancel.child_token())?;
        let sender_task = self.sending_task(rx_sender, cancel.child_token())?;
        let control_task = self.control_task(rx_control, cancel)?;
        let tx_events = self.tx_events.clone();
        Ok(Box::pin(async move {
            if let Err(err) = select! {
                res = streams_task => {
                    debug!(target: logs::targets::SERVER, "[main]:: finished on streams_task");
                    match res {
                        Ok(res) => match res {
                            Ok(_) => Ok(()),
                            Err(err) => Err(err)
                        },
                        Err(err) => Err(Error::JoinError(err))
                    }                },
                res = accepting_task => {
                    debug!(target: logs::targets::SERVER, "[main]:: finished on accepting_task");
                    match res {
                        Ok(res) => match res {
                            Ok(_) => Ok(()),
                            Err(err) => Err(err)
                        },
                        Err(err) => Err(Error::JoinError(err))
                    }
                },
                res = messages_task => {
                    debug!(target: logs::targets::SERVER, "[main]:: finished on messages_task");
                    match res {
                        Ok(res) => match res {
                            Ok(_) => Ok(()),
                            Err(err) => Err(err)
                        },
                        Err(err) => Err(Error::JoinError(err))
                    }
                },
                res = sender_task => {
                    debug!(target: logs::targets::SERVER, "[main]:: finished on sender_task");
                    match res {
                        Ok(res) => match res {
                            Ok(_) => Ok(()),
                            Err(err) => Err(err)
                        },
                        Err(err) => Err(Error::JoinError(err))
                    }
                },
                res = control_task => {
                    debug!(target: logs::targets::SERVER, "[main]:: finished on control_task");
                    match res {
                        Ok(res) => res,
                        Err(err) => Err(Error::JoinError(err))
                    }

                },
            } {
                error!(target: logs::targets::SERVER, "[main]:: finished with error: {}", err);
            } else {
                info!(target: logs::targets::SERVER, "[main]:: finished");
            }
            if let Err(e) = tx_events.send(Events::Shutdown) {
                warn!(
                    target: logs::targets::SERVER,
                    "Cannot send Events::Shutdown . Error: {}", e
                );
            }
            Ok(())
        }))
    }

    fn observer(&mut self) -> Result<UnboundedReceiver<Events<Error>>, Error> {
        if let Some(rx_events) = self.rx_events.take() {
            Ok(rx_events)
        } else {
            Err(Error::ObserverAlreadyTaken)
        }
    }

    fn sender(&self) -> UnboundedSender<Sending> {
        self.tx_sender.clone()
    }

    fn control(&self) -> UnboundedSender<ServerControl> {
        self.tx_control.clone()
    }
}
