use super::{
    consumer::{connect, protocol, protocol::StructDecode, Context, Options, ReconnectionStrategy},
    stat::StatEvent,
    test,
};
use clibri_transport_client::{
    client,
    client::Client,
    errors::Error,
    options::{ConnectionType, Distributor, Options as ClientOptions},
};
use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;

pub async fn run(
    addr: &str,
    timeout: u64,
    tx_stat: UnboundedSender<StatEvent>,
    shutdown: bool,
    distributor: bool,
) -> Result<(), String> {
    let socket_addr = addr.parse::<SocketAddr>().map_err(|e| e.to_string())?;
    let client = Client::new(ClientOptions {
        connection: if !distributor {
            ConnectionType::Direct(socket_addr)
        } else {
            ConnectionType::Distributor(Distributor::new(socket_addr))
        },
    });
    let context = Context::new(tx_stat.clone());
    let disconnected = context.disconnected.clone();
    let mut options = Options::defualt(protocol::StructA::defaults());
    options.request_timeout = timeout;
    options.reconnection = ReconnectionStrategy::DoNotReconnect;
    let connected = context.connected.child_token();
    let broadcast_received = context.broadcast_received.child_token();
    let finish = context.finish.child_token();
    let consumer_holder = connect::<Client, Error, client::Control>(client, context, options)
        .await
        .map_err(|e| e.to_string())?;
    connected.cancelled().await;
    let mut consumer = consumer_holder.get().await.map_err(|e| e.to_string())?;
    if !shutdown {
        test::executor(
            "Test StructA Request",
            timeout,
            test::test_request_structa::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test StructC Request",
            timeout,
            test::test_request_structc::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test StructD Request",
            timeout,
            test::test_request_structd::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test GroupB::StructA Request",
            timeout,
            test::test_request_groupb_structa::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test GroupA::StructB Request",
            timeout,
            test::test_request_groupa_structb::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test GroupB::GroupC::StructA Request",
            timeout,
            test::test_request_groupb_groupc_structa::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test GroupB::GroupC::StructB Request",
            timeout,
            test::test_request_groupb_groupc_structb::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test GroupA::StructA Request",
            timeout,
            test::test_request_groupa_structa::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test StructEmpty Request",
            timeout,
            test::test_request_structempty::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor_no_res(
            "Waiting for broadcast messages",
            timeout,
            broadcast_received.cancelled(),
        )
        .await?;
        test::executor(
            "Test StructF Request",
            timeout,
            test::test_request_structf::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor_no_res(
            "Waiting for last broadcast message",
            timeout,
            finish.cancelled(),
        )
        .await?;
        consumer.shutdown().await.map_err(|e| e.to_string())?;
        tx_stat
            .send(StatEvent::ConsumerDone)
            .map_err(|e| e.to_string())?;
    } else {
        consumer
            .beacon_beacons_shutdownserver(protocol::Beacons::ShutdownServer {})
            .await
            .map_err(|e| e.to_string())?;
        test::executor_no_res(
            "Waiting disconnection (because server is down)",
            timeout,
            disconnected.cancelled(),
        )
        .await?;
    }
    Ok(())
}
