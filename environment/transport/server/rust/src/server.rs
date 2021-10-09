use super::{
    channel::{Control as ConnectionControl, Messages},
    connection::Connection,
    env as server_env,
    errors::Error,
    handshake::Handshake as HandshakeInterface,
    options::{Distributor, Listener, Options, Ports},
    stat::Stat,
};
use async_trait::async_trait;
use fiber::{env, env::logs, server};
use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Method, Request as HttpRequest, Response as HttpResponse, Server as HttpServer,
    StatusCode,
};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::{
    join,
    net::{TcpListener, TcpStream},
    select,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task,
};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{Request, Response},
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

type CreatePortListenerRequest = (u16, oneshot::Sender<()>);
type PortSender = UnboundedSender<oneshot::Sender<Option<u16>>>;
type PortReceiver = UnboundedReceiver<oneshot::Sender<Option<u16>>>;
type MonitorSender = UnboundedSender<(u16, MonitorEvent)>;
type MonitorReceiver = UnboundedReceiver<(u16, MonitorEvent)>;

pub enum MonitorEvent {
    Connected,
    Disconnected,
}

pub struct Handshake;

impl HandshakeInterface for Handshake {}

enum InternalChannel {
    GetPort(oneshot::Sender<Option<u16>>),
    DelegatePort(oneshot::Sender<Option<u16>>),
    Insert(u16, (u32, CancellationToken), oneshot::Sender<()>),
    MonitorEvent(MonitorEvent, u16, oneshot::Sender<()>),
    InsertControl(
        Uuid,
        UnboundedSender<ConnectionControl>,
        oneshot::Sender<()>,
    ),
    RemoveControl(Uuid, oneshot::Sender<()>),
    Send(Vec<u8>, Option<Uuid>, oneshot::Sender<()>),
    Disconnect(Option<Uuid>, oneshot::Sender<()>),
    PrintStat(oneshot::Sender<()>),
    StatRecievedBytes(usize),
    StatListenerCreated,
    StatListenerDestroyed,
    StatConnecting,
}

#[derive(Clone)]
pub struct Control {
    api: InternalAPI,
}

impl Control {
    pub async fn print_stat(&self) -> Result<(), Error> {
        self.api.print_stat().await
    }
}
#[async_trait]
impl server::Control<Error> for Control {
    async fn shutdown(&self) -> Result<(), Error> {
        debug!(
            target: logs::targets::SERVER,
            "server::Control::Shutdown has been called"
        );
        self.api.lock();
        self.api.disconnect_all().await?;
        if server_env::is_debug_mode() {
            if let Err(err) = self.api.print_stat().await {
                warn!(target: logs::targets::SERVER, "fail to print stat: {}", err);
            }
        }
        self.api.shutdown();
        Ok(())
    }
    async fn send(&self, buffer: Vec<u8>, client: Option<Uuid>) -> Result<(), Error> {
        self.api.send(buffer, client).await
    }
    async fn disconnect(&self, client: Uuid) -> Result<(), Error> {
        self.api.disconnect(client).await
    }
    async fn disconnect_all(&self) -> Result<(), Error> {
        self.api.disconnect_all().await
    }
}

#[derive(Clone)]
struct InternalAPI {
    tx_api: UnboundedSender<InternalChannel>,
    shutdown: CancellationToken,
    locked: CancellationToken,
}

impl InternalAPI {
    pub async fn get_port(&self) -> Result<Option<u16>, Error> {
        let (tx_resolve, rx_resolve): (
            oneshot::Sender<Option<u16>>,
            oneshot::Receiver<Option<u16>>,
        ) = oneshot::channel();
        self.tx_api
            .send(InternalChannel::GetPort(tx_resolve))
            .map_err(|e| Error::Channel(format!("Fail do api::get_port; error: {}", e)))?;
        rx_resolve.await.map_err(|e| {
            Error::Channel(format!("Fail get response for api::get_port; error: {}", e))
        })
    }

    pub async fn delegate_port(&self) -> Result<Option<u16>, Error> {
        let (tx_resolve, rx_resolve): (
            oneshot::Sender<Option<u16>>,
            oneshot::Receiver<Option<u16>>,
        ) = oneshot::channel();
        self.tx_api
            .send(InternalChannel::DelegatePort(tx_resolve))
            .map_err(|e| Error::Channel(format!("Fail do api::delegate_port; error: {}", e)))?;
        rx_resolve.await.map_err(|e| {
            Error::Channel(format!(
                "Fail get response for api::delegate_port; error: {}",
                e
            ))
        })
    }

    pub async fn insert(&self, port: u16, data: (u32, CancellationToken)) -> Result<(), Error> {
        let (tx_resolve, rx_resolve): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        self.tx_api
            .send(InternalChannel::Insert(port, data, tx_resolve))
            .map_err(|e| Error::Channel(format!("Fail do api::insert; error: {}", e)))?;
        rx_resolve
            .await
            .map_err(|e| Error::Channel(format!("Fail get response for api::insert; error: {}", e)))
    }

    pub async fn monitor_event(&self, port: u16, event: MonitorEvent) -> Result<(), Error> {
        let (tx_resolve, rx_resolve): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        self.tx_api
            .send(InternalChannel::MonitorEvent(event, port, tx_resolve))
            .map_err(|e| Error::Channel(format!("Fail do api::monitor_event; error: {}", e)))?;
        rx_resolve.await.map_err(|e| {
            Error::Channel(format!(
                "Fail get response for api::monitor_event; error: {}",
                e
            ))
        })
    }

    pub async fn insert_control(
        &self,
        uuid: Uuid,
        tx_control: UnboundedSender<ConnectionControl>,
    ) -> Result<(), Error> {
        let (tx_resolve, rx_resolve): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        self.tx_api
            .send(InternalChannel::InsertControl(uuid, tx_control, tx_resolve))
            .map_err(|e| Error::Channel(format!("Fail do api::insert_control; error: {}", e)))?;
        rx_resolve.await.map_err(|e| {
            Error::Channel(format!(
                "Fail get response for api::insert_control; error: {}",
                e
            ))
        })
    }

    pub async fn remove_control(&self, uuid: Uuid) -> Result<(), Error> {
        let (tx_resolve, rx_resolve): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        self.tx_api
            .send(InternalChannel::RemoveControl(uuid, tx_resolve))
            .map_err(|e| Error::Channel(format!("Fail do api::remove_control; error: {}", e)))?;
        rx_resolve.await.map_err(|e| {
            Error::Channel(format!(
                "Fail get response for api::remove_control; error: {}",
                e
            ))
        })
    }

    pub async fn send(&self, buffer: Vec<u8>, uuid: Option<Uuid>) -> Result<(), Error> {
        let (tx_resolve, rx_resolve): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        self.tx_api
            .send(InternalChannel::Send(buffer, uuid, tx_resolve))
            .map_err(|e| Error::Channel(format!("Fail do api::send; error: {}", e)))?;
        rx_resolve
            .await
            .map_err(|e| Error::Channel(format!("Fail get response for api::send; error: {}", e)))
    }

    pub async fn disconnect(&self, uuid: Uuid) -> Result<(), Error> {
        let (tx_resolve, rx_resolve): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        self.tx_api
            .send(InternalChannel::Disconnect(Some(uuid), tx_resolve))
            .map_err(|e| Error::Channel(format!("Fail do api::disconnect; error: {}", e)))?;
        rx_resolve.await.map_err(|e| {
            Error::Channel(format!(
                "Fail get response for api::disconnect; error: {}",
                e
            ))
        })
    }

    pub async fn disconnect_all(&self) -> Result<(), Error> {
        let (tx_resolve, rx_resolve): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        self.tx_api
            .send(InternalChannel::Disconnect(None, tx_resolve))
            .map_err(|e| Error::Channel(format!("Fail do api::disconnect; error: {}", e)))?;
        rx_resolve.await.map_err(|e| {
            Error::Channel(format!(
                "Fail get response for api::disconnect; error: {}",
                e
            ))
        })
    }

    pub async fn print_stat(&self) -> Result<(), Error> {
        let (tx_resolve, rx_resolve): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        self.tx_api
            .send(InternalChannel::PrintStat(tx_resolve))
            .map_err(|e| Error::Channel(format!("Fail do api::print_stat; error: {}", e)))?;
        rx_resolve.await.map_err(|e| {
            Error::Channel(format!(
                "Fail get response for api::print_stat; error: {}",
                e
            ))
        })
    }

    pub fn stat_recieved_bytes(&self, bytes: usize) -> Result<(), Error> {
        self.tx_api
            .send(InternalChannel::StatRecievedBytes(bytes))
            .map_err(|e| Error::Channel(format!("Fail do api::recieved_bytes; error: {}", e)))
    }

    pub fn stat_listener_created(&self) -> Result<(), Error> {
        self.tx_api
            .send(InternalChannel::StatListenerCreated)
            .map_err(|e| Error::Channel(format!("Fail do api::listener_created; error: {}", e)))
    }

    pub fn stat_listener_destroyed(&self) -> Result<(), Error> {
        self.tx_api
            .send(InternalChannel::StatListenerDestroyed)
            .map_err(|e| {
                Error::Channel(format!(
                    "Fail do api::stat_listener_destroyed; error: {}",
                    e
                ))
            })
    }

    pub fn stat_connecting(&self) -> Result<(), Error> {
        self.tx_api
            .send(InternalChannel::StatConnecting)
            .map_err(|e| Error::Channel(format!("Fail do api::stat_connecting; error: {}", e)))
    }

    pub fn shutdown(&self) {
        self.shutdown.cancel();
    }

    pub fn lock(&self) {
        if !self.is_locked() {
            self.locked.cancel();
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked.is_cancelled()
    }
}

pub struct Server {
    options: Options,
    tx_events: UnboundedSender<server::Events<Error>>,
    rx_events: Option<UnboundedReceiver<server::Events<Error>>>,
    rx_api: Option<UnboundedReceiver<InternalChannel>>,
    api: InternalAPI,
    control: Control,
}

impl Server {
    pub fn new(options: Options) -> Self {
        env::logs::init();
        let (tx_events, rx_events): (
            UnboundedSender<server::Events<Error>>,
            UnboundedReceiver<server::Events<Error>>,
        ) = unbounded_channel();
        let (tx_api, rx_api): (
            UnboundedSender<InternalChannel>,
            UnboundedReceiver<InternalChannel>,
        ) = unbounded_channel();
        let api = InternalAPI {
            tx_api,
            shutdown: CancellationToken::new(),
            locked: CancellationToken::new(),
        };
        Self {
            options,
            tx_events,
            rx_events: Some(rx_events),
            rx_api: Some(rx_api),
            api: api.clone(),
            control: Control { api },
        }
    }

    async fn api_task(
        &self,
        mut rx_api: UnboundedReceiver<InternalChannel>,
        options: Option<Distributor>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let mut connections: HashMap<u16, (u32, CancellationToken)> = HashMap::new();
        let mut controlls: HashMap<Uuid, UnboundedSender<ConnectionControl>> = HashMap::new();
        let mut stat: Stat = Stat::new();
        let tx_events = self.tx_events.clone();
        info!(target: logs::targets::SERVER, "[task: api]:: started");
        select! {
            res = async {
                let remove_control = |
                    controlls: &mut HashMap<Uuid, UnboundedSender<ConnectionControl>>,
                    stat: &mut Stat,
                    uuid: Uuid| {
                    if controlls.remove(&uuid).is_some() {
                        stat.disconnected();
                        debug!(target: logs::targets::SERVER, "{}:: Channel of connection has been removed", uuid);
                        stat.alive(controlls.len());
                        tx_events.send(server::Events::Disconnected(uuid)).map_err(|e| Error::Channel(e.to_string()))?;
                    } else {
                        error!(target: logs::targets::SERVER, "{}:: Fail to find channel of connection to remove it", uuid);
                    }
                    Ok(())
                };
                while let Some(msg) = rx_api.recv().await {
                    match msg {
                        InternalChannel::GetPort(tx_resolve) => {
                            if let Some(options) = options.as_ref() {
                                tx_resolve
                                .send(connections.iter().find_map(|(port, (count, _cancel))| {
                                    if count < &options.connections_per_port {
                                        Some(port.to_owned())
                                    } else {
                                        None
                                    }
                                }))
                                .map_err(|_| {
                                    Error::Channel(String::from(
                                        "Fail handle InternalChannel::GetPort command",
                                    ))
                                })?;
                            } else {
                                return Err(Error::Channel(String::from(
                                    "Fail handle InternalChannel::GetPort command: no options",
                                )));
                            }
                        }
                        InternalChannel::DelegatePort(tx_resolve) => {
                            if let Some(options) = options.as_ref() {
                                tx_resolve
                                .send(match options.ports.clone() {
                                    Ports::List(ports) => {
                                        info!(
                                            target: logs::targets::SERVER,
                                            "looking for port from a list"
                                        );
                                        let mut free: Option<u16> = None;
                                        for port in ports.iter() {
                                            if !connections.contains_key(port) {
                                                free = Some(port.to_owned());
                                                break;
                                            }
                                        }
                                        free
                                    }
                                    Ports::Range(range) => {
                                        info!(
                                            target: logs::targets::SERVER,
                                            "looking for port from a range"
                                        );
                                        let mut free: Option<u16> = None;
                                        for port in range {
                                            if !connections.contains_key(&port) {
                                                free = Some(port);
                                                break;
                                            }
                                        }
                                        free
                                    }
                                })
                                .map_err(|_| {
                                    Error::Channel(String::from(
                                        "Fail handle InternalChannel::DelegatePort command",
                                    ))
                                })?;
                            } else {
                                return Err(Error::Channel(String::from(
                                    "Fail handle InternalChannel::DelegatePort command: no options",
                                )));
                            }

                        }
                        InternalChannel::Insert(port, data, tx_resolve) => {
                            connections.insert(port, data);
                            tx_resolve.send(()).map_err(|_| {
                                Error::Channel(String::from(
                                    "Fail handle InternalChannel::Insert command",
                                ))
                            })?;
                        }
                        InternalChannel::MonitorEvent(event, port, tx_resolve) => {
                            match event {
                                MonitorEvent::Connected => {
                                    if let Some((count, _cancel)) = connections.get_mut(&port) {
                                        *count += 1;
                                    }
                                }
                                MonitorEvent::Disconnected => {
                                    if let Some((count, cancel)) = connections.get_mut(&port) {
                                        *count -= 1;
                                        if count == &0 {
                                            cancel.cancel();
                                            connections.remove(&port);
                                        }
                                    }
                                }
                            };
                            tx_resolve.send(()).map_err(|_| {
                                Error::Channel(String::from(
                                    "Fail handle InternalChannel::MonitorEvent command",
                                ))
                            })?;
                        },
                        InternalChannel::InsertControl(uuid, tx_control, tx_resolve) => {
                            controlls.entry(uuid).or_insert(tx_control);
                            stat.connected();
                            stat.alive(controlls.len());
                            tx_events.send(server::Events::Connected(uuid)).map_err(|e| Error::Channel(e.to_string()))?;
                            tx_resolve.send(()).map_err(|_| {
                                Error::Channel(String::from(
                                    "Fail handle InternalChannel::InsertControl command",
                                ))
                            })?;
                            debug!(target: logs::targets::SERVER, "Controll of connection has been added");
                        }
                        InternalChannel::RemoveControl(uuid, tx_resolve) => {
                            remove_control(&mut controlls, &mut stat, uuid.to_owned())?;
                            tx_resolve.send(()).map_err(|_| {
                                Error::Channel(String::from(
                                    "Fail handle InternalChannel::RemoveControl command",
                                ))
                            })?;
                        }
                        InternalChannel::Send(buffer, uuid, tx_resolve) => {
                            let len = buffer.len();
                            if let Some(uuid) = uuid {
                                if let Some(control) = controlls.get_mut(&uuid) {
                                    if let Err(e) = control.send(ConnectionControl::Send(buffer)) {
                                        error!(target: logs::targets::SERVER, "{}:: Fail to close connection due error: {}", uuid, e);
                                        tx_events.send(server::Events::Error(Some(uuid), format!("{}", e))).map_err(|e| Error::Channel(e.to_string()))?;
                                    } else { stat.sent_bytes(len); }
                                }
                            } else {
                                for (uuid, control) in controlls.iter_mut() {
                                    if let Err(e) = control.send(ConnectionControl::Send(buffer.clone())) {
                                        error!(target: logs::targets::SERVER, "{}:: Fail to close connection due error: {}", uuid, e);
                                        tx_events.send(server::Events::Error(Some(*uuid), format!("{}", e))).map_err(|e| Error::Channel(e.to_string()))?;
                                    } else { stat.sent_bytes(len); }
                                }
                            }
                            tx_resolve.send(()).map_err(|_| {
                                Error::Channel(String::from(
                                    "Fail handle InternalChannel::Send command",
                                ))
                            })?;
                        }
                        InternalChannel::Disconnect(uuid, tx_resolve) => {
                            if let Some(uuid) = uuid.as_ref() {
                                // Disconnect client
                                if let Some(control) = controlls.get(&uuid) {
                                    let (tx_shutdown_resolve, rx_shutdown_resolve): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
                                    if let Err(e) = control.send(ConnectionControl::Disconnect(tx_shutdown_resolve)) {
                                        error!(target: logs::targets::SERVER, "{}:: Fail to send close connection command due error: {}", uuid, e);
                                        tx_events.send(server::Events::Error(Some(*uuid), format!("{}", e))).map_err(|e| Error::Channel(e.to_string()))?;
                                    } else if rx_shutdown_resolve.await.is_err() {
                                        error!(target: logs::targets::SERVER, "{}:: Fail get disconnect confirmation", uuid);
                                        tx_events.send(server::Events::Error(Some(*uuid), String::from("Fail get disconnect confirmation"))).map_err(|e| Error::Channel(e.to_string()))?;
                                    }
                                    remove_control(&mut controlls, &mut stat, uuid.to_owned())?;
                                } else {
                                    error!(target: logs::targets::SERVER, "Command Disconnect has been gotten. But cannot find client: {}", uuid);
                                    tx_events.send(server::Events::ServerError(Error::CreateWS(format!("Command Disconnect has been gotten. But cannot find client: {}", uuid)))).map_err(|e| Error::Channel(e.to_string()))?;
                                }
                            } else {
                                // Disconnect all
                                let mut uuids: Vec<Uuid> = vec![];
                                for (uuid, control) in controlls.iter() {
                                    let (tx_shutdown_resolve, rx_shutdown_resolve): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
                                    if let Err(e) = control.send(ConnectionControl::Disconnect(tx_shutdown_resolve)) {
                                        error!(target: logs::targets::SERVER, "{}:: Fail to send close connection command due error: {}", uuid, e);
                                        if let Err(err) = tx_events.send(server::Events::Error(Some(*uuid), format!("{}", e))) {
                                            error!(target: logs::targets::SERVER, "{}:: Cannot send event Error; error: {}", uuid, err);
                                        }
                                    } else if rx_shutdown_resolve.await.is_err() {
                                        error!(target: logs::targets::SERVER, "{}:: Fail get disconnect confirmation", uuid);
                                        if let Err(err) = tx_events.send(server::Events::Error(Some(*uuid), String::from("Fail get disconnect confirmation"))) {
                                            error!(target: logs::targets::SERVER, "{}:: Cannot send event Error; error: {}", uuid, err);
                                        }
                                    }
                                    uuids.push(uuid.to_owned());
                                }
                                for uuid in uuids {
                                    remove_control(&mut controlls, &mut stat, uuid.to_owned())?;
                                }
                            }
                            tx_resolve.send(()).map_err(|_| {
                                Error::Channel(String::from(
                                    "Fail handle InternalChannel::Disconnect command",
                                ))
                            })?;
                    }
                        InternalChannel::PrintStat(tx_resolve) => {
                            stat.print();
                            tx_resolve.send(()).map_err(|_| {
                                Error::Channel(String::from(
                                    "Fail handle InternalChannel::PrintStat command",
                                ))
                            })?;
                        }
                        InternalChannel::StatRecievedBytes(bytes) => {
                            stat.recieved_bytes(bytes);
                        }
                        InternalChannel::StatListenerCreated => {
                            stat.listener_created();
                        }
                        InternalChannel::StatListenerDestroyed => {
                            stat.listener_destroyed();
                        }
                        InternalChannel::StatConnecting => {
                            stat.connecting();
                        }
                    }
                }
                info!(target: logs::targets::SERVER, "[task: api]:: finished");
                Ok(())
            } => res,
            _ = cancel.cancelled() => {
                info!(target: logs::targets::SERVER, "[task: api]:: canceled");
                Ok(())
            }
        }
    }

    async fn streams_task(
        addr: SocketAddr,
        tx_tcp_stream: UnboundedSender<TcpStream>,
        tx_events: UnboundedSender<server::Events<Error>>,
        api: InternalAPI,
        mut tx_listener_ready: Option<oneshot::Sender<()>>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        info!(target: logs::targets::SERVER, "[task: streams]:: started");
        let listener = match TcpListener::bind(addr).await {
            Ok(listener) => {
                api.stat_listener_created()?;
                debug!(
                    target: logs::targets::SERVER,
                    "server has been started on {}", addr
                );
                listener
            }
            Err(e) => {
                error!(
                    target: logs::targets::SERVER,
                    "Fail to start server on {}. Error: {}", addr, e
                );
                tx_events
                    .send(server::Events::ServerError(Error::Create(format!("{}", e))))
                    .map_err(|e| Error::Channel(e.to_string()))?;
                return Err(Error::Create(format!("{}", e)));
            }
        };
        if let Some(tx_listener_ready) = tx_listener_ready.take() {
            tx_listener_ready.send(()).map_err(|e| {
                Error::Channel(format!("fail to send ready listener signal: {:?}", e))
            })?;
        } else {
            tx_events
                .send(server::Events::Ready)
                .map_err(|e| Error::Channel(e.to_string()))?;
        }
        let res = select! {
            res = async {
                loop {
                    let stream = match listener.accept().await {
                        Ok((stream, _addr)) => {
                            debug!(target: logs::targets::SERVER, "Getting request to connect from: {}", _addr);
                            api.stat_connecting()?;
                            // TODO: middleware to confirm acception
                            stream
                        },
                        Err(e) => {
                            warn!(target: logs::targets::SERVER, "Cannot accept connection. Error: {}", e);
                            tx_events.send(server::Events::ServerError(Error::AcceptStream(format!("{}", e)))).map_err(|e| Error::Channel(e.to_string()))?;
                            continue;
                        }
                    };
                    if let Err(e) = tx_tcp_stream.send(stream) {
                        warn!(target: logs::targets::SERVER, "Cannot share stream. Error: {}", e);
                        tx_events.send(server::Events::ServerError(Error::AcceptStream(format!("{}", e)))).map_err(|e| Error::Channel(e.to_string()))?;
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
        api.stat_listener_destroyed()?;
        res
    }

    async fn accepting_task(
        &self,
        tx_messages: UnboundedSender<Messages>,
        mut rx_tcp_stream: UnboundedReceiver<TcpStream>,
        monitor: Option<MonitorSender>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let tx_events = self.tx_events.clone();
        let api = self.api.clone();
        info!(target: logs::targets::SERVER, "[task: accepting]:: started");
        select! {
            res = async {
                while let Some(stream) = rx_tcp_stream.recv().await {
                    debug!(target: logs::targets::SERVER, "New stream has been gotten");
                    let port = stream.local_addr().map_err(|e| Error::SocketAddr(e.to_string()))?.port();
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
                    let uuid = Uuid::new_v4();
                    let control = match Connection::new(uuid).attach(ws, tx_events.clone(), tx_messages.clone(), monitor.clone(), port).await {
                        Ok(control) => control,
                        Err(e) => {
                            warn!(target: logs::targets::SERVER, "Cannot create ws connection. Error: {}", e);
                            tx_events.send(server::Events::ServerError(Error::CreateWS(e.to_string()))).map_err(|e| Error::Channel(e.to_string()))?;
                            continue;
                        }
                    };
                    api.insert_control(uuid, control).await?;
                }
                Ok(())
            } => res,
            _ = cancel.cancelled() => {
                info!(target: logs::targets::SERVER, "[task: accepting]:: shutdown called");
                Ok(())
            }
        }
    }

    async fn messages_task(
        &self,
        mut rx_messages: UnboundedReceiver<Messages>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let tx_events = self.tx_events.clone();
        let api = self.api.clone();
        info!(target: logs::targets::SERVER, "[task: messages]:: started");
        select! {
            res = async {
                while let Some(msg) = rx_messages.recv().await {
                    if api.is_locked() {
                        continue;
                    }
                    match msg {
                        Messages::Binary { uuid, buffer } => {
                            api.stat_recieved_bytes(buffer.len())?;
                            tx_events.send(server::Events::Received(uuid, buffer)).map_err(|e| Error::Channel(e.to_string()))?;
                        },
                        Messages::Disconnect { uuid, code } => {
                            debug!(target: logs::targets::SERVER, "{}:: Client wants to disconnect (code: {:?})", uuid, code);
                            api.remove_control(uuid).await?;
                        },
                        Messages::Error { uuid, error } => {
                            tx_events.send(server::Events::Error(Some(uuid), format!("{:?}", error).to_string())).map_err(|e| Error::Channel(e.to_string()))?;
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
    }

    async fn distributor_task(
        &self,
        options: Distributor,
        tx_tcp_stream: UnboundedSender<TcpStream>,
        mut rx_port_request: PortReceiver,
        mut rx_monitor: MonitorReceiver,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let addr = options.addr;
        let api = self.api.clone();
        let tx_events = self.tx_events.clone();
        let (tx_create_listener, mut rx_create_listener): (
            UnboundedSender<CreatePortListenerRequest>,
            UnboundedReceiver<CreatePortListenerRequest>,
        ) = unbounded_channel();
        let result = select! {
            res = async {
                while let Some(tx_response) = rx_port_request.recv().await {
                    if api.is_locked() {
                        continue;
                    }
                    info!(target: logs::targets::SERVER, "request for available port is gotten");
                    let mut port = api.get_port().await?;
                    if let Some(port) = port.take() {
                        if let Err(err) = tx_response.send(Some(port)) {
                            error!(target: logs::targets::SERVER, "fail to send response about available port: {:?}", err);
                        }
                    } else {
                        info!(target: logs::targets::SERVER, "no open sockets has been found");
                        let mut port: Option<u16> = api.delegate_port().await?;
                        if let Some(port_value) = port.as_ref() {
                            let (tx_listener_ready, rx_listener_ready): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
                            tx_create_listener.send((port_value.to_owned(), tx_listener_ready)).map_err(|e| Error::Distributing(format!("fail to create new listener: {}", e)))?;
                            if let Err(err) = rx_listener_ready.await {
                                error!(target: logs::targets::SERVER, "fail to get response from listener: {:?}", err);
                                port = None;
                            }
                        }
                        if let Err(err) = tx_response.send(port) {
                            warn!(target: logs::targets::SERVER, "fail to response on port request: {:?}", err);
                        }
                    }
                }
                Ok::<(), Error>(())
            } => res,
            res = async {
                while let Some((port, tx_listener_ready)) = rx_create_listener.recv().await {
                    if api.is_locked() {
                        continue;
                    }
                    let socket_addr = format!("{}:{}", addr, port).parse::<SocketAddr>().map_err(|e| Error::SocketAddr(e.to_string()))?;
                    let tx_events = tx_events.clone();
                    let api = api.clone();
                    let tx_tcp_stream = tx_tcp_stream.clone();
                    let close_child_token = cancel.child_token();
                    api.insert(port, (0, close_child_token.clone())).await?;
                    task::spawn(async move {
                        debug!(target: logs::targets::SERVER, "streams tasks for port {} created", port);
                        if let Err(err) = Self::streams_task(
                            socket_addr,
                            tx_tcp_stream,
                            tx_events,
                            api,
                            Some(tx_listener_ready),
                            close_child_token
                        ).await {
                            warn!(target: logs::targets::SERVER, "streams tasks is finished with error: {:?}", err);
                        }
                        debug!(target: logs::targets::SERVER, "streams tasks for port {} destroyed", port);
                    });
                }
                Ok::<(), Error>(())
            } => res,
            res = async {
                while let Some((port, event)) = rx_monitor.recv().await {
                    api.monitor_event(port, event).await?;
                }
                Ok::<(), Error>(())
            } => res,
        };
        result
    }

    async fn http_response(
        request: HttpRequest<Body>,
        tx_port_request: PortSender,
    ) -> Result<HttpResponse<Body>, Error> {
        let response = |status: StatusCode, data: String| -> Result<HttpResponse<Body>, Error> {
            HttpResponse::builder()
                .status(status)
                .header(header::CONTENT_TYPE, "text/plain; charset=UTF-8")
                .body(Body::from(data))
                .map_err(|e| Error::BodyParsing(e.to_string()))
        };
        match *request.method() {
            Method::GET => {
                let (tx_port_response, rx_port_response): (
                    oneshot::Sender<Option<u16>>,
                    oneshot::Receiver<Option<u16>>,
                ) = oneshot::channel();
                info!(
                    target: logs::targets::SERVER,
                    "available port has been requested"
                );
                if let Err(_err) = tx_port_request.send(tx_port_response) {
                    error!(target: logs::targets::SERVER, "fail request available port");
                    response(StatusCode::NO_CONTENT, String::from("error"))
                } else if let Ok(port) = rx_port_response.await {
                    if let Some(port) = port {
                        info!(
                            target: logs::targets::SERVER,
                            "available port has been granded: {}", port
                        );
                        response(StatusCode::OK, port.to_string())
                    } else {
                        warn!(target: logs::targets::SERVER, "no available ports");
                        response(StatusCode::OK, String::from("0"))
                    }
                } else {
                    error!(
                        target: logs::targets::SERVER,
                        "cannot get available port data"
                    );
                    response(StatusCode::NO_CONTENT, String::from("nope"))
                }
            }
            _ => response(StatusCode::NO_CONTENT, String::from("nope")),
        }
    }

    async fn http_task(
        &self,
        addr: SocketAddr,
        tx_port_request: PortSender,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        debug!(target: logs::targets::SERVER, "starting distributor");
        let service = make_service_fn(move |_| {
            let tx_port_request = tx_port_request.clone();
            async {
                Ok::<_, Error>(service_fn(move |request| {
                    Self::http_response(request, tx_port_request.to_owned())
                }))
            }
        });
        let server = HttpServer::bind(&addr).serve(service);
        debug!(
            target: logs::targets::SERVER,
            "distributor listening on http://{}", addr
        );
        let graceful = server.with_graceful_shutdown(async { cancel.cancelled().await });
        self.tx_events
            .send(server::Events::Ready)
            .map_err(|e| Error::Channel(e.to_string()))?;
        graceful
            .await
            .map_err(|e| Error::HttpServer(e.to_string()))?;
        debug!(target: logs::targets::SERVER, "shutdown distributor");
        Ok(())
    }

    async fn direct(&mut self, addr: SocketAddr) -> Result<(), Error> {
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
        let cancel = self.api.shutdown.clone();
        let rx_api = if let Some(rx_api) = self.rx_api.take() {
            rx_api
        } else {
            return Err(Error::FailTakeAPI);
        };
        let tx_events = self.tx_events.clone();
        let (streams_task, api_task, accepting_task, messages_task) = join!(
            Self::streams_task(
                addr,
                tx_tcp_stream,
                self.tx_events.clone(),
                self.api.clone(),
                None,
                cancel.child_token()
            ),
            self.api_task(rx_api, None, cancel.child_token()),
            self.accepting_task(tx_messages, rx_tcp_stream, None, cancel.child_token()),
            self.messages_task(rx_messages, cancel.child_token()),
        );
        tx_events
            .send(server::Events::Shutdown)
            .map_err(|err| Error::Channel(err.to_string()))?;
        if let Err(err) = api_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: api_task finished with error: {}", err
            );
            return Err(err);
        }
        if let Err(err) = streams_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: streams_task finished with error: {}", err
            );
            return Err(err);
        }
        if let Err(err) = accepting_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: accepting_task finished with error: {}", err
            );
            return Err(err);
        }
        if let Err(err) = messages_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: messages_task finished with error: {}", err
            );
            return Err(err);
        }
        debug!(
            target: logs::targets::SERVER,
            "[main]:: all tasks are finished without errors"
        );
        Ok(())
    }

    async fn distributor(&mut self, options: Distributor) -> Result<(), Error> {
        info!(
            target: logs::targets::SERVER,
            "[main]: will create main task"
        );
        info!(target: logs::targets::SERVER, "[main]: runtime is created");
        let (tx_tcp_stream, rx_tcp_stream): (
            UnboundedSender<TcpStream>,
            UnboundedReceiver<TcpStream>,
        ) = unbounded_channel();
        let (tx_port_request, rx_port_request): (PortSender, PortReceiver) = unbounded_channel();
        let (tx_messages, rx_messages): (UnboundedSender<Messages>, UnboundedReceiver<Messages>) =
            unbounded_channel();
        let (tx_monitor, rx_monitor): (MonitorSender, MonitorReceiver) = unbounded_channel();
        let cancel = self.api.shutdown.clone();
        let rx_api = if let Some(rx_api) = self.rx_api.take() {
            rx_api
        } else {
            return Err(Error::FailTakeAPI);
        };
        let tx_events = self.tx_events.clone();
        let (http_task, api_task, distributor_task, accepting_task, messages_task) = join!(
            self.api_task(rx_api, Some(options.clone()), cancel.child_token()),
            self.http_task(options.distributor, tx_port_request, cancel.child_token()),
            self.distributor_task(
                options,
                tx_tcp_stream,
                rx_port_request,
                rx_monitor,
                cancel.child_token()
            ),
            self.accepting_task(
                tx_messages,
                rx_tcp_stream,
                Some(tx_monitor),
                cancel.child_token()
            ),
            self.messages_task(rx_messages, cancel.child_token()),
        );
        tx_events
            .send(server::Events::Shutdown)
            .map_err(|err| Error::Channel(err.to_string()))?;
        if let Err(err) = api_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: api_task finished with error: {}", err
            );
            return Err(err);
        }
        if let Err(err) = http_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: http_task finished with error: {}", err
            );
            return Err(err);
        }
        if let Err(err) = distributor_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: distributor_task finished with error: {}", err
            );
            return Err(err);
        }
        if let Err(err) = accepting_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: accepting_task finished with error: {}", err
            );
            return Err(err);
        }
        if let Err(err) = messages_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: messages_task finished with error: {}", err
            );
            return Err(err);
        }
        debug!(
            target: logs::targets::SERVER,
            "[main]:: all tasks are finished without errors"
        );
        Ok(())
    }
}

#[async_trait]
impl server::Impl<Error, Control> for Server {
    async fn listen(&mut self) -> Result<(), Error> {
        let options = self.options.clone();
        match options.listener {
            Listener::Direct(addr) => self.direct(addr).await,
            Listener::Distributor(opts) => self.distributor(opts).await,
        }
    }

    fn observer(&mut self) -> Result<UnboundedReceiver<server::Events<Error>>, Error> {
        if let Some(rx_events) = self.rx_events.take() {
            Ok(rx_events)
        } else {
            Err(Error::ObserverAlreadyTaken)
        }
    }

    fn control(&self) -> Control {
        self.control.clone()
    }
}
