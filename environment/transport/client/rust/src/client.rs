use super::{
    errors::Error,
    events::{Event, Message},
    options::{ConnectionType, Options},
};
use fiber::env::logs;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use hyper::{Client as HttpClient, StatusCode, Uri};
use log::{debug, error};
use std::net::SocketAddr;
use tokio::{
    net::TcpStream,
    select,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task,
};
use tokio_tungstenite::{
    connect_async, tungstenite::Message as WsMessage, MaybeTlsStream, WebSocketStream,
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub type Done = Result<(), Error>;
pub type ConnectReturn = (
    UnboundedReceiver<Event>,
    oneshot::Receiver<Result<(), Error>>,
);
#[derive(Debug, Clone)]
pub enum ToSend {
    Binary(Vec<u8>),
    Text(String),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Client {
    options: Options,
    cancel: CancellationToken,
    tx_sender: UnboundedSender<ToSend>,
    uuid: Uuid,
}

impl Client {
    pub fn new(options: Options, mut cancel: Option<CancellationToken>) -> Self {
        let (tx_sender, _): (UnboundedSender<ToSend>, UnboundedReceiver<ToSend>) =
            unbounded_channel();
        Self {
            options,
            tx_sender,
            cancel: if let Some(cancel) = cancel.take() {
                cancel
            } else {
                CancellationToken::new()
            },
            uuid: Uuid::new_v4(),
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn send(&self, msg: ToSend) -> Result<(), Error> {
        if let Err(err) = self.tx_sender.send(msg) {
            Err(Error::Channel(format!(
                "fail to send message; error: {:?}",
                err
            )))
        } else {
            Ok(())
        }
    }

    pub fn stop(&self) {
        self.cancel.cancel();
    }

    async fn direct_connection(
        addr: SocketAddr,
        tx_events: UnboundedSender<Event>,
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
        tx_events: UnboundedSender<Event>,
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
        tx_events: UnboundedSender<Event>,
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
        mut rx_sender: UnboundedReceiver<ToSend>,
        cancel: CancellationToken,
    ) -> Result<(), Error> {
        let result = select! {
            res = async {
                while let Some(msg) = rx_sender.recv().await {
                    match msg {
                        ToSend::Binary(buffer) => writer
                            .send(WsMessage::Binary(buffer))
                            .await
                            .map_err(|e| Error::Write(e.to_string()))?,
                        ToSend::Text(txt) => writer
                            .send(WsMessage::Text(txt))
                            .await
                            .map_err(|e| Error::Write(e.to_string()))?,
                        ToSend::Ping(buffer) => writer
                            .send(WsMessage::Ping(buffer))
                            .await
                            .map_err(|e| Error::Write(e.to_string()))?,
                        ToSend::Pong(buffer) => writer
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

    pub async fn connect(&mut self) -> Result<ConnectReturn, Error> {
        let cancel = self.cancel.clone();
        let (tx_events, rx_events): (UnboundedSender<Event>, UnboundedReceiver<Event>) =
            unbounded_channel();
        let socket = match self.options.connection {
            ConnectionType::Direct(addr) => {
                Self::direct_connection(addr, tx_events.clone()).await?
            }
            ConnectionType::Distributor(addr) => {
                Self::distributor_connection(addr, tx_events.clone()).await?
            }
        };
        let (tx_done, rx_done): (oneshot::Sender<Done>, oneshot::Receiver<Done>) =
            oneshot::channel();
        let (tx_sender, rx_sender): (UnboundedSender<ToSend>, UnboundedReceiver<ToSend>) =
            unbounded_channel();
        let (writer, reader) = socket.split();
        self.tx_sender = tx_sender;
        task::spawn(async move {
            let res = select! {
                res = Self::reader_task(reader, tx_events.clone(), cancel.child_token()) => res,
                res = Self::writer_task(writer, rx_sender, cancel) => res,
            };
            if let Err(err) = tx_events.send(Event::Disconnected) {
                error!(
                    target: logs::targets::CLIENT,
                    "fail to send event Disconnected; error: {:?}", err
                );
            }
            if let Err(err) = res {
                if let Err(err) = tx_done.send(Err(err)) {
                    error!(
                        target: logs::targets::CLIENT,
                        "fail to send done signal; error: {:?}", err
                    );
                }
            } else if let Err(err) = tx_done.send(Ok(())) {
                error!(
                    target: logs::targets::CLIENT,
                    "fail to send done signal; error: {:?}", err
                );
            }
        });
        Ok((rx_events, rx_done))
    }
}
