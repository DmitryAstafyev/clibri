use super::{
    broadcast, events, identification, pack, producer::Control, scope::Scope, Context, EmitterError,
};
use clibri::server;
use uuid::Uuid;

pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: &identification::Filter<'_>,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    let mut scope: Scope<'_, E, C> = Scope::new(context, control, identification, filter);
    events::disconnected::emit(&mut scope)
        .await
        .map_err(EmitterError::Emitting)?;
    scope.call().await;
    Ok(())
}
