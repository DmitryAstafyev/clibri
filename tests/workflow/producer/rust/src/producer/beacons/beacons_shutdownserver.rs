use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use tokio::{
    task,
    time::{sleep, Duration},
};

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    beacon: &protocol::Beacons::ShutdownServer,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    context.stats.remove(&identification.uuid());
    let shutdown_token = control.get_shutdown_token();
    task::spawn(async move {
        // Shutdown after delay to let server send confirmation of getting
        // this beacon
        sleep(Duration::from_millis(1000)).await;
        shutdown_token.cancel();
    });
    Ok(())
}
