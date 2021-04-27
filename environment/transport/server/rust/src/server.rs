use super::{
    channel::{Control, Messages},
    stat::{Stat},
    connection::Connection,
    handshake::Handshake as HandshakeInterface,
    tools,
};
use fiber::{
    logger::Logger,
    server::{errors::Errors, events::Events, interface::Interface, control::Control as ServerControl},
};
use futures::{executor, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::{
    io::{
        AsyncRead,
        AsyncWrite,
    },
    net::{TcpListener, TcpStream},
    sync::oneshot::{channel, Receiver, Sender},
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::{spawn, JoinHandle},
    join,
    select,
    runtime::{Runtime},
};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::{
        handshake::server::{Request, Response},
        protocol::frame::coding::CloseCode,
    },
    WebSocketStream,
};
use uuid::Uuid;

pub struct Handshake;

impl HandshakeInterface for Handshake {}


pub struct Server {
    addr: String,
    controlls: Arc<RwLock<HashMap<Uuid, UnboundedSender<Control>>>>,
    stat: Arc<RwLock<Stat>>,
}

impl Server {

    pub fn new(addr: String) -> Self {
        Self {
            addr,
            controlls: Arc::new(RwLock::new(HashMap::new())),
            stat: Arc::new(RwLock::new(Stat::new()))
        }
    }

    pub fn print_stat(&self) -> () {
        if let Ok(stat) = self.stat.write() {
            stat.print();
        }
    }

}

impl Interface for Server {

    fn listen(
        &mut self,
        events: UnboundedSender<Events>,
        mut sending: UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
        mut control: Option<UnboundedReceiver<ServerControl>>,
    ) -> Result<(), String> {
        let rt  = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(tools::logger.err(&format!("Fail to create runtime executor. Error: {}", e)))
            },
        };
        rt.block_on(async move {
            tools::logger.verb("[main]: runtime is created");
            let addr: String = self.addr.clone();
            let events_cl = events.clone();
            let stat = self.stat.clone();
            let send_event = move |event: Events| {
                match event {
                    Events::Error(_, _) | Events::ConnectionError(_, _) | Events::ServerError(_) =>
                        if let Ok(mut stat) = stat.write() { stat.errors(); },
                    _ => {}
                };
                if let Err(e) = events_cl.send(event) {
                    tools::logger.warn(&format!("Cannot send event. Error: {}", e));
                }
            };
            let (tx_tcp_stream, mut rx_tcp_stream): (
                UnboundedSender<TcpStream>,
                UnboundedReceiver<TcpStream>,
            ) = unbounded_channel();
            let (tx_streams_task_sd, mut rx_streams_task_sd): (
                Sender<()>,
                Receiver<()>,
            ) = channel();
            let stat = self.stat.clone();
            let streams_task: JoinHandle<()> = spawn(async move {
                tools::logger.verb("[task: streams]:: started");
                let listener = match TcpListener::bind(addr).await {
                    Ok(listener) => listener,
                    Err(e) => {
                        send_event(Events::ServerError(Errors::Create(
                            tools::logger.warn(&format!("Fail to start server. Error: {}", e)),
                        )));
                        return;
                    }
                };
                send_event(Events::Ready);
                select! {
                    _ = async {
                        loop {
                            let stream = match listener.accept().await {
                                Ok((stream, _addr)) => {
                                    tools::logger.debug(&format!("Getting request to connect from: {}", _addr));
                                    if let Ok(mut stat) = stat.write() { stat.connecting(); }
                                    // TODO: middleware to confirm acception
                                    stream
                                },
                                Err(e) => {
                                    send_event(Events::ServerError(Errors::AcceptStream(
                                        tools::logger.warn(&format!("Cannot accept connection. Error: {}", e)),
                                    )));
                                    continue;
                                }
                            };
                            if let Err(e) = tx_tcp_stream.send(stream) {
                                send_event(Events::ServerError(Errors::AcceptStream(
                                    tools::logger.warn(&format!("Cannot share stream. Error: {}", e)),
                                )));
                                break;
                            }
                        }
                    } => {},
                    _ = rx_streams_task_sd => {}
                };
                tools::logger.verb("[task: streams]:: finished");
            });
            let events_cl = events.clone();
            let stat = self.stat.clone();
            let send_event = move |event: Events| {
                match event {
                    Events::Error(_, _) | Events::ConnectionError(_, _) | Events::ServerError(_) =>
                        if let Ok(mut stat) = stat.write() { stat.errors(); },
                    _ => {}
                };
                if let Err(e) = events_cl.send(event) {
                    tools::logger.warn(&format!("Cannot send event. Error: {}", e));
                }
            };
            let controlls = self.controlls.clone();
            let connection_events = events.clone();
            let stat = self.stat.clone();
            let (tx_messages, mut rx_messages): (
                UnboundedSender<Messages>,
                UnboundedReceiver<Messages>,
            ) = unbounded_channel();
            let (tx_accepting_task_sd, mut rx_accepting_task_sd): (
                Sender<()>,
                Receiver<()>,
            ) = channel();
            let accepting_task: JoinHandle<()> = spawn(async move {
                tools::logger.verb("[task: accepting]:: started");
                select! {
                    _ = async {
                        while let Some(stream) = rx_tcp_stream.recv().await {
                            tools::logger.debug("New stream has been gotten");
                            let ws = match accept_hdr_async(stream, |req: &Request, response: Response| {
                                Handshake::accept(req, response)
                            })
                            .await
                            {
                                Ok(ws) => ws,
                                Err(e) => {
                                    tools::logger.warn(&format!("Fail to accept stream due error: {:?}", e));
                                    continue;
                                }
                            };
                            tools::logger.debug("Connection has been accepted");
                            if let Ok(mut stat) = stat.write() { stat.connected(); }
                            let uuid = Uuid::new_v4();
                            let control = match Connection::new(uuid).attach(ws, connection_events.clone(), tx_messages.clone()).await {
                                Ok(control) => control,
                                Err(e) => {
                                    send_event(Events::ServerError(Errors::CreateWS(
                                        tools::logger
                                            .warn(&format!("Cannot create ws connection. Error: {}", e)),
                                    )));
                                    continue;
                                }
                            };
                            match controlls.write() {
                                Ok(mut controlls) => {
                                    controlls.entry(uuid).or_insert(control);
                                    if let Ok(mut stat) = stat.write() { stat.alive(controlls.len()); }
                                    tools::logger.debug("Controll of connection has been added");
                                    send_event(Events::Connected(uuid.clone()));
                                },
                                Err(e) => {
                                    send_event(Events::ServerError(Errors::CreateWS(
                                        tools::logger.err(&format!("Fail get controlls due error: {}", e)),
                                    )));
                                    continue;
                                }
                            };
                        }
                    } => {},
                    _ = rx_accepting_task_sd => {}
                };
                tools::logger.verb("[task: accepting]:: finished");
            });
            let controlls = self.controlls.clone();
            let events_cl = events.clone();
            let stat = self.stat.clone();
            let send_event = move |event: Events| {
                match event {
                    Events::Error(_, _) | Events::ConnectionError(_, _) | Events::ServerError(_) =>
                        if let Ok(mut stat) = stat.write() { stat.errors(); },
                    _ => {}
                };
                if let Err(e) = events_cl.send(event) {
                    tools::logger.warn(&format!("Cannot send event. Error: {}", e));
                }
            };
            let stat = self.stat.clone();
            let (tx_messages_task_sd, mut rx_messages_task_sd): (
                Sender<()>,
                Receiver<()>,
            ) = channel();
            let messages_task: JoinHandle<()> = spawn(async move {
                tools::logger.verb("[task: messages]:: started");
                select! {
                    _ = async {
                        while let Some(msg) = rx_messages.recv().await {
                            match msg {
                                Messages::Binary { uuid, buffer } => {
                                    if let Ok(mut stat) = stat.write() { stat.recieved_bytes(buffer.len()); }
                                    send_event(Events::Received(uuid, buffer))
                                },
                                Messages::Disconnect { uuid, code } => {
                                    if let Ok(mut stat) = stat.write() { stat.disconnected(); }
                                    match controlls.write() {
                                        Ok(mut controlls) => {
                                            if let Some(_control) = controlls.remove(&uuid) {
                                                tools::logger.debug(&format!("{}:: Channel of connection has been removed", uuid));
                                                if let Ok(mut stat) = stat.write() { stat.alive(controlls.len()); }
                                                send_event(Events::Disconnected(uuid.clone()));
                                            } else {
                                                tools::logger.err(&format!("{}:: Fail to find channel of connection to remove it", uuid));
                                            }
                                        },
                                        Err(e) => send_event(Events::Error(Some(uuid), tools::logger.err(&format!("{}:: Cannot get access to controllers. Error: {}", uuid, e)))),
                                    };
                                },
                                Messages::Error { uuid, error } => send_event(Events::Error(Some(uuid), format!("{:?}", error).to_string()))
                            }
                        }
                    } => {},
                    _ = rx_messages_task_sd => {}
                };
                tools::logger.verb("[task: messages]:: finished");
            });
            let controlls = self.controlls.clone();
            let events_cl = events.clone();
            let stat = self.stat.clone();
            let send_event = move |event: Events| {
                match event {
                    Events::Error(_, _) | Events::ConnectionError(_, _) | Events::ServerError(_) =>
                        if let Ok(mut stat) = stat.write() { stat.errors(); },
                    _ => {}
                };
                if let Err(e) = events_cl.send(event) {
                    tools::logger.warn(&format!("Cannot send event. Error: {}", e));
                }
            };
            let stat = self.stat.clone();
            let (tx_sender_task_sd, mut rx_sender_task_sd): (
                Sender<()>,
                Receiver<()>,
            ) = channel();
            let sender_task: JoinHandle<()> = spawn(async move {
                tools::logger.verb("[task: sender]:: started");
                select! {
                    _ = async {
                        while let Some((buffer, uuid)) = sending.recv().await {
                            match controlls.write() {
                                Ok(mut controlls) => {
                                    let len = buffer.len();
                                    if let Some(uuid) = uuid {
                                        if let Some(control) = controlls.get_mut(&uuid) {
                                            if let Err(e) = control.send(Control::Send(buffer)) {
                                                send_event(Events::Error(Some(uuid), tools::logger.err(&format!("{}:: Fail to close connection due error: {}", uuid, e))))
                                            } else {
                                                if let Ok(mut stat) = stat.write() { stat.sent_bytes(len); }
                                            }
                                        }
                                    } else {
                                        for (uuid, control) in controlls.iter_mut() {
                                            if let Err(e) = control.send(Control::Send(buffer.clone())) {
                                                send_event(Events::Error(Some(*uuid), tools::logger.err(&format!("{}:: Fail to close connection due error: {}", uuid, e))))
                                            } else {
                                                if let Ok(mut stat) = stat.write() { stat.sent_bytes(len); }
                                            }
                                        }
                                    }               
                                },
                                Err(e) => {
                                    send_event(Events::Error(None, tools::logger.err(&format!("Cannot get access to controllers. Error: {}", e))));
                                    break;
                                },
                            };
                        }                       
                    } => {},
                    _ = rx_sender_task_sd => {}
                };
                tools::logger.verb("[task: sender]:: finished");
            });
            let control_task: JoinHandle<()> = spawn(async move {
                tools::logger.verb("[task: control]:: started");
                if let Some(mut control) = control {
                    match control.recv().await {
                        Some(msg) => match msg {
                            ServerControl::Shutdown => {
                                if let Err(e) = tx_streams_task_sd.send(()) { tools::logger.err("fail call shutdown for streams task"); }
                                if let Err(e) = tx_accepting_task_sd.send(()) { tools::logger.err("fail call shutdown for accepting task"); }
                                if let Err(e) = tx_messages_task_sd.send(()) { tools::logger.err("fail call shutdown for messages task"); }
                                if let Err(e) = tx_sender_task_sd.send(()) { tools::logger.err("fail call shutdown for sender task"); }
                            }
                        },
                        _ => {

                        },
                    }
                    
                }
                tools::logger.verb("[task: control]:: finished");
            });
            select! {
                _ = streams_task => {},
                _ = accepting_task => {},
                _ = messages_task => {},
                _ = sender_task => {},
                _ = control_task => {},
            };
            tools::logger.verb("[main]:: finished");
        });
        Ok(())
    }

}

impl Drop for Server {
    fn drop(&mut self) {
        println!(" ====> Dropping Server!");
    }
}