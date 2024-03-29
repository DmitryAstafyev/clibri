use super::{events, identification, producer::Control, Context, EmitterError, ProducerError};
use clibri::server;
use uuid::Uuid;

pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    error: ProducerError<E>,
    uuid: Option<Uuid>,
    context: &mut Context,
    identification: Option<&identification::Identification>,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    events::error::emit(error, uuid, context, identification, control)
        .await
        .map_err(EmitterError::Emitting)
}
