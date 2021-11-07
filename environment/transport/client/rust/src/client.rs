use super::{
    errors::Error,
    options::{ConnectionType, Options},
};
use async_trait::async_trait;
use fiber::{
    client,
    client::{Control as ClientControl, Event, Impl, Message},
    env,
    env::logs,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use hyper::{Client as HttpClient, StatusCode, Uri};
use log::{debug, error, warn};
use std::net::SocketAddr;
use tokio::{
    net::TcpStream,
    select,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_tungstenite::{
    connect_async, tungstenite::Message as WsMessage, MaybeTlsStream, WebSocketStream,
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub type Done = Result<(), Error>;
pub type ConnectReturn = (
    UnboundedReceiver<Event<Error>>,
    oneshot::Receiver<Result<(), Error>>,
);

#[derive(Debug, Clone)]
pub struct Control {
    shutdown: CancellationToken,
    tx_sender: UnboundedSender<Message>,
}

impl Control {
    pub fn get_shutdown_token(&self) -> CancellationToken {
        self.shutdown.clone()
    }
}

#[async_trait]
impl client::Control<Error> for Control {
    async fn shutdown(&self) -> Result<(), Error> {
        self.shutdown.cancel();
        Ok(())
    }
    async fn send(&self, msg: Message) -> Result<(), Error> {
        self.tx_sender
            .send(msg)
            .map_err(|e| Error::Channel(e.to_string()))
    }
}
#[derive(Debug)]
pub struct Client {
    options: Options,
    uuid: Uuid,
    control: Control,
    rx_sender: Option<UnboundedReceiver<Message>>,
    rx_events: Option<UnboundedReceiver<Event<Error>>>,
    tx_events: UnboundedSender<Event<Error>>,
}

impl Client {
    pub fn new(options: Options) -> Self {
        let (tx_sender, rx_sender): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
            unbounded_channel();
        let (tx_events, rx_events): (
            UnboundedSender<Event<Error>>,
            UnboundedReceiver<Event<Error>>,
        ) = unbounded_channel();
        Self {
            options,
            tx_events,
            rx_sender: Some(rx_sender),
            rx_events: Some(rx_events),
            uuid: Uuid::new_v4(),
            control: Control {
                tx_sender,
                shutdown: CancellationToken::new(),
            },
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    async fn direct_connection(
        addr: SocketAddr,
        tx_events: UnboundedSender<Event<Error>>,
    ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Error> {
        let addr_str = format!("ws://{}:{}", addr.ip(), addr.port());
        match connect_async(&addr_str).await {
            Ok((ws, _)) => {
                if let Err(err) = tx_events.send(Event::Connected(addr)) {
                    error!(
                        target: logs::targets::CLIENT,
                        "fail to emit Event::Connected: {:?}", err
                    );
                }
                Ok(ws)
            }
            Err(err) => Err(Error::Connecting(format!(
                "Fail to connect to {}; error: {}",
                addr_str, err
            ))),
        }
    }

    async fn distributor_connection(
        addr: SocketAddr,
        tx_events: UnboundedSender<Event<Error>>,
    ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Error> {
        let distributor_url = format!("http://{}:{}", addr.ip(), addr.port())
            .parse::<Uri>()
            .map_err(|e| Error::DistributorUrl(e.to_string()))?;
        debug!(
            target: logs::targets::CLIENT,
            "requesting port for websocket connection from {}", distributor_url
        );
        let http_client = HttpClient::new();
        let response = http_client
            .get(distributor_url)
            .await
            .map_err(|e| Error::HttpRequest(e.to_string()))?;
        if response.status() != StatusCode::OK {
            error!(
                target: logs::targets::CLIENT,
                "has been gotten status: {}",
                response.status()
            );
            return Err(Error::DistributorFail);
        }
        let buffer = hyper::body::to_bytes(response)
            .await
            .map_err(|e| Error::DistributorResponse(e.to_string()))?;
        let port: u16 = String::from(
            std::str::from_utf8(&buffer.to_vec())
                .map_err(|e| Error::DistributorResponse(e.to_string()))?,
        )
        .parse()
        .map_err(|_| Error::DistributorInvalidResponse)?;
        let addr_str = format!("ws://{}:{}", addr.ip(), port);
        debug!(
            target: logs::targets::CLIENT,
            "will try to connect to: {}", addr_str
        );
        match connect_async(&addr_str).await {
            Ok((ws, _)) => {
                let addr = format!("{}:{}", addr.ip(), port)
                    .parse::<SocketAddr>()
                    .map_err(|e| Error::SocketAddr(e.to_string()))?;
                if let Err(err) = tx_events.send(Event::Connected(addr)) {
                    error!(
                        target: logs::targets::CLIENT,
                        "fail to emit Event::Connected: {:?}", err
                    );
                }
                Ok(ws)
            }
            Err(err) => Err(Error::Connecting(format!(
                "Fail to connect to {}; error: {}",
                addr_str, err
            ))),
        }
    }

    async fn reader_task(
        mut reader: SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
        tx_events: UnboundedSender<Event<Error>>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        debug!(target: logs::targets::CLIENT, "reader_task is started");
        let result = select! {
            res = async {
                while let Some(msg) = reader.next().await {
                    match msg {
                        Ok(msg) => match msg {
                            WsMessage::Binary(buffer) => tx_events
                                .send(Event::Message(Message::Binary(buffer)))
                                .map_err(|e| Error::Channel(e.to_string()))?,
                            WsMessage::Text(txt) => tx_events
                                .send(Event::Message(Message::Text(txt)))
                                .map_err(|e| Error::Channel(e.to_string()))?,
                            WsMessage::Ping(buffer) => tx_events
                                .send(Event::Message(Message::Ping(buffer)))
                                .map_err(|e| Error::Channel(e.to_string()))?,
                            WsMessage::Pong(buffer) => tx_events
                                .send(Event::Message(Message::Pong(buffer)))
                                .map_err(|e| Error::Channel(e.to_string()))?,
                            WsMessage::Close(frame) => {
                                if let Some(frame) = frame {
                                    debug!(
                                        target: logs::targets::CLIENT,
                                        "connection would be closed with {:?}", frame
                                    );
                                } else {
                                    debug!(
                                        target: logs::targets::CLIENT,
                                        "connection would be closed without CloseFrame"
                                    );
                                }
                            }
                        },
                        Err(err) => {
                            return Err(Error::Read(err.to_string()));
                        }
                    }
                }
                Ok(())
            } => res,
            _ = cancel.cancelled() => Ok(())
        };
        debug!(target: logs::targets::CLIENT, "reader_task is finished");
        result
    }

    async fn writer_task(
        mut writer: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>,
        mut rx_sender: UnboundedReceiver<Message>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let result = select! {
            res = async {
                while let Some(msg) = rx_sender.recv().await {
                    match msg {
                        Message::Binary(buffer) => writer
                            .send(WsMessage::Binary(buffer))
                            .await
                            .map_err(|e| Error::Write(e.to_string()))?,
                        Message::Text(txt) => writer
                            .send(WsMessage::Text(txt))
                            .await
                            .map_err(|e| Error::Write(e.to_string()))?,
                        Message::Ping(buffer) => writer
                            .send(WsMessage::Ping(buffer))
                            .await
                            .map_err(|e| Error::Write(e.to_string()))?,
                        Message::Pong(buffer) => writer
                            .send(WsMessage::Pong(buffer))
                            .await
                            .map_err(|e| Error::Write(e.to_string()))?,
                    }
                }
                Ok(())
            } => res,
            _ = cancel.cancelled() => Ok(())
        };
        if let Err(err) = writer.close().await {
            error!(
                target: logs::targets::CLIENT,
                "fail to close socket; error: {}", err
            );
        }
        result
    }

    fn reinit(&mut self) {
        let (tx_sender, rx_sender): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
            unbounded_channel();
        let (tx_events, rx_events): (
            UnboundedSender<Event<Error>>,
            UnboundedReceiver<Event<Error>>,
        ) = unbounded_channel();
        self.tx_events = tx_events;
        self.rx_events = Some(rx_events);
        self.rx_sender = Some(rx_sender);
        self.control = Control {
            tx_sender,
            shutdown: CancellationToken::new(),
        };
        debug!(target: logs::targets::CLIENT, "client has been reinited");
    }
}

#[async_trait]
impl Impl<Error, Control> for Client {
    async fn connect(&mut self) -> Result<(), Error> {
        env::logs::init();
        debug!(target: logs::targets::CLIENT, "client is started");
        let cancel = self.control.get_shutdown_token();
        let socket = match self.options.connection {
            ConnectionType::Direct(addr) => {
                Self::direct_connection(addr, self.tx_events.clone()).await
            }
            ConnectionType::Distributor(addr) => {
                Self::distributor_connection(addr, self.tx_events.clone()).await
            }
        };
        let socket = match socket {
            Ok(socket) => socket,
            Err(err) => {
                self.reinit();
                warn!(
                    target: logs::targets::CLIENT,
                    "client is finished with error: {}", err
                );
                if let Err(err) = self.tx_events.send(Event::Error(err.clone())) {
                    error!(
                        target: logs::targets::CLIENT,
                        "fail to send event Error; error: {:?}", err
                    );
                }
                return Err(err);
            }
        };
        let (writer, reader) = socket.split();
        let rx_sender = if let Some(rx_sender) = self.rx_sender.take() {
            rx_sender
        } else {
            return Err(Error::SenderAlreadyTaken);
        };
        let res = select! {
            res = Self::reader_task(reader, self.tx_events.clone(), cancel.child_token()) => res,
            res = Self::writer_task(writer, rx_sender, cancel) => res,
        };
        if let Err(err) = self.tx_events.send(Event::Disconnected) {
            error!(
                target: logs::targets::CLIENT,
                "fail to send event Disconnected; error: {:?}", err
            );
        }
        self.control.shutdown().await?;
        self.reinit();
        debug!(target: logs::targets::CLIENT, "client is finished");
        res
    }

    fn observer(&mut self) -> Result<UnboundedReceiver<Event<Error>>, Error> {
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
