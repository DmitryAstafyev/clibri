use super::{
    channel::{Control, Messages},
    connection::Connection,
    errors::Error,
    handshake::Handshake as HandshakeInterface,
    options::{Distributor, Listener, Options, Ports},
    stat::Stat,
};
use async_trait::async_trait;
use fiber::{
    env,
    env::logs,
    server::{
        control::Control as ServerControl,
        events::Events,
        interface::{Interface, Sending},
    },
};
use futures::lock::Mutex;
use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Method, Request as HttpRequest, Response as HttpResponse, Server as HttpServer,
    StatusCode,
};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
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

pub struct Server {
    options: Options,
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
    pub fn new(options: Options) -> Self {
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
            options,
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

    async fn streams_task(
        addr: SocketAddr,
        tx_tcp_stream: UnboundedSender<TcpStream>,
        tx_events: UnboundedSender<Events<Error>>,
        stat: Arc<RwLock<Stat>>,
        mut tx_listener_ready: Option<oneshot::Sender<()>>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        info!(target: logs::targets::SERVER, "[task: streams]:: started");
        let listener = match TcpListener::bind(addr).await {
            Ok(listener) => {
                if let Ok(mut stat) = stat.write() {
                    stat.listener_created();
                }
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
                    .send(Events::ServerError(Error::Create(format!("{}", e))))
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
                .send(Events::Ready)
                .map_err(|e| Error::Channel(e.to_string()))?;
        }
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
        if let Ok(mut stat) = stat.write() {
            stat.listener_destroyed();
        }
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
        let stat = self.stat.clone();
        let controlls = self.controlls.clone();
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
                    if let Ok(mut stat) = stat.write() { stat.connected(); }
                    let uuid = Uuid::new_v4();
                    let control = match Connection::new(uuid).attach(ws, tx_events.clone(), tx_messages.clone(), monitor.clone(), port).await {
                        Ok(control) => control,
                        Err(e) => {
                            warn!(target: logs::targets::SERVER, "Cannot create ws connection. Error: {}", e);
                            tx_events.send(Events::ServerError(Error::CreateWS(e.to_string()))).map_err(|e| Error::Channel(e.to_string()))?;
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
    }

    async fn messages_task(
        &self,
        mut rx_messages: UnboundedReceiver<Messages>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let tx_events = self.tx_events.clone();
        let stat = self.stat.clone();
        let controlls = self.controlls.clone();
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
    }

    async fn sending_task(
        &self,
        mut rx_sending: UnboundedReceiver<Sending>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let tx_events = self.tx_events.clone();
        let stat = self.stat.clone();
        let controlls = self.controlls.clone();
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
    }

    async fn control_task(
        &self,
        mut control: UnboundedReceiver<ServerControl>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let tx_events = self.tx_events.clone();
        let controlls = self.controlls.clone();
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
    }

    async fn distributor_task(
        &self,
        options: Distributor,
        tx_tcp_stream: UnboundedSender<TcpStream>,
        mut rx_port_request: PortReceiver,
        mut rx_monitor: MonitorReceiver,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let connections: Arc<Mutex<HashMap<u16, (u32, CancellationToken)>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let connections_per_port = options.connections_per_port;
        let ports = options.ports;
        let addr = options.addr;
        let connections_rc = connections.clone();
        let stat_rc = self.stat.clone();
        let tx_events = self.tx_events.clone();
        let (tx_create_listener, mut rx_create_listener): (
            UnboundedSender<CreatePortListenerRequest>,
            UnboundedReceiver<CreatePortListenerRequest>,
        ) = unbounded_channel();
        let result = select! {
            res = async {
                while let Some(tx_response) = rx_port_request.recv().await {
                    info!(target: logs::targets::SERVER, "request for available port is gotten");
                    let connections = connections_rc.lock().await;
                    let mut port = connections.iter().find_map(|(port, (count, _cancel))| {
                        if count < &connections_per_port {
                            Some(port.to_owned())
                        } else {
                            None
                        }
                    });
                    drop(connections);
                    if let Some(port) = port.take() {
                        if let Err(err) = tx_response.send(Some(port)) {
                            error!(target: logs::targets::SERVER, "fail to send response about available port: {:?}", err);
                        }
                    } else {
                        info!(target: logs::targets::SERVER, "no open sockets has been found");
                        let mut port: Option<u16> = match ports.clone() {
                            Ports::List(ports) => {
                                info!(target: logs::targets::SERVER, "looking for port from a list");
                                let connections = connections_rc.lock().await;
                                let mut free: Option<u16> = None;
                                for port in ports.iter() {
                                    if !connections.contains_key(port) {
                                        free = Some(port.to_owned());
                                        break;
                                    }
                                }
                                drop(connections);
                                free
                            },
                            Ports::Range(range) => {
                                info!(target: logs::targets::SERVER, "looking for port from a range");
                                let connections = connections_rc.lock().await;
                                let mut free: Option<u16> = None;
                                for port in range {
                                    if !connections.contains_key(&port) {
                                        free = Some(port);
                                        break;
                                    }
                                }
                                drop(connections);
                                free
                            }
                        };
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
                    let socket_addr = format!("{}:{}", addr, port).parse::<SocketAddr>().map_err(|e| Error::SocketAddr(e.to_string()))?;
                    let tx_events = tx_events.clone();
                    let stat_rc = stat_rc.clone();
                    let tx_tcp_stream = tx_tcp_stream.clone();
                    let close_child_token = cancel.child_token();
                    let mut connections = connections_rc.lock().await;
                    connections.insert(port, (0, close_child_token.clone()));
                    drop(connections);
                    task::spawn(async move {
                        debug!(target: logs::targets::SERVER, "streams tasks for port {} created", port);
                        if let Err(err) = Self::streams_task(
                            socket_addr,
                            tx_tcp_stream,
                            tx_events,
                            stat_rc,
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
                    let mut connections = connections_rc.lock().await;
                    match event {
                        MonitorEvent::Connected => if let Some((count, _cancel)) = connections.get_mut(&port) {
                            *count += 1;
                        },
                        MonitorEvent::Disconnected => if let Some((count, cancel)) = connections.get_mut(&port) {
                            *count -= 1;
                            if count == &0 {
                                cancel.cancel();
                                connections.remove(&port);
                            }
                        }
                    };
                    drop(connections);
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
            .send(Events::Ready)
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
        let tx_events = self.tx_events.clone();
        let (streams_task, accepting_task, messages_task, sending_task, control_task) = join!(
            Self::streams_task(
                addr,
                tx_tcp_stream,
                self.tx_events.clone(),
                self.stat.clone(),
                None,
                cancel.child_token()
            ),
            self.accepting_task(tx_messages, rx_tcp_stream, None, cancel.child_token()),
            self.messages_task(rx_messages, cancel.child_token()),
            self.sending_task(rx_sender, cancel.child_token()),
            self.control_task(rx_control, cancel),
        );
        tx_events
            .send(Events::Shutdown)
            .map_err(|err| Error::Channel(err.to_string()))?;
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
        if let Err(err) = sending_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: sending_task finished with error: {}", err
            );
            return Err(err);
        }
        if let Err(err) = control_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: control_task finished with error: {}", err
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
        let tx_events = self.tx_events.clone();
        let (
            http_task,
            distributor_task,
            accepting_task,
            messages_task,
            sending_task,
            control_task,
        ) = join!(
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
            self.sending_task(rx_sender, cancel.child_token()),
            self.control_task(rx_control, cancel),
        );
        tx_events
            .send(Events::Shutdown)
            .map_err(|err| Error::Channel(err.to_string()))?;
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
        if let Err(err) = sending_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: sending_task finished with error: {}", err
            );
            return Err(err);
        }
        if let Err(err) = control_task {
            error!(
                target: logs::targets::SERVER,
                "[main]:: control_task finished with error: {}", err
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
impl Interface<Error> for Server {
    async fn listen(&mut self) -> Result<(), Error> {
        let options = self.options.clone();
        match options.listener {
            Listener::Direct(addr) => self.direct(addr).await,
            Listener::Distributor(opts) => self.distributor(opts).await,
        }
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
