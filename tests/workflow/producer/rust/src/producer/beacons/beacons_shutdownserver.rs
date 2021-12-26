use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::stop;
use clibri::server;
use std::pin::Pin;
use tokio::{
    task,
    time::{sleep, Duration},
};

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    beacon: &protocol::Beacons::ShutdownServer,
    scope: &mut Scope<'_, E, C>,
) -> Result<(), String> {
    scope.context.stats.remove(&scope.identification.uuid());
    let control = scope.control.clone();
    scope.deferred(Box::pin(async move {
        if let Err(err) = control.shutdown(false).await {
            stop!("Fail to shutdown; error: {:?}", err);
        }
    }));
    Ok(())
}
