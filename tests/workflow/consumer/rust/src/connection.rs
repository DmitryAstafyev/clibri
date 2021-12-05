use super::{
    consumer::{connect, protocol, protocol::StructDecode, Context, Options},
    stat::StatEvent,
    test,
};
use clibri_transport_client::{
    client,
    client::Client,
    errors::Error,
    options::{ConnectionType, Options as ClientOptions},
};
use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;

pub async fn run(
    addr: &str,
    tx_stat: UnboundedSender<StatEvent>,
    shutdown: bool,
) -> Result<(), String> {
    let socket_addr = addr.parse::<SocketAddr>().map_err(|e| e.to_string())?;
    let client = Client::new(ClientOptions {
        connection: ConnectionType::Direct(socket_addr),
    });
    let context = Context::new(tx_stat.clone());
    let disconnected = context.disconnected.clone();
    let options = Options::defualt(protocol::StructA::defaults());
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
            10000,
            test::test_request_structa::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test StructC Request",
            10000,
            test::test_request_structc::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test StructD Request",
            10000,
            test::test_request_structd::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test GroupB::StructA Request",
            10000,
            test::test_request_groupb_structa::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test GroupA::StructB Request",
            10000,
            test::test_request_groupa_structb::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test GroupB::GroupC::StructA Request",
            10000,
            test::test_request_groupb_groupc_structa::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test GroupB::GroupC::StructB Request",
            10000,
            test::test_request_groupb_groupc_structb::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test GroupA::StructA Request",
            10000,
            test::test_request_groupa_structa::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor(
            "Test StructEmpty Request",
            10000,
            test::test_request_structempty::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor_no_res(
            "Waiting for broadcast messages",
            10000,
            broadcast_received.cancelled(),
        )
        .await?;
        test::executor(
            "Test StructF Request",
            10000,
            test::test_request_structf::execute(&mut consumer, &tx_stat),
        )
        .await?;
        test::executor_no_res(
            "Waiting for last broadcast message",
            10000,
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
            10000,
            disconnected.cancelled(),
        )
        .await?;
    }
    Ok(())
}
