use super::{events, producer::Control, Context, EmitterError, ProducerError};
use uuid::Uuid;

pub async fn emit<E: std::error::Error>(
    error: ProducerError<E>,
    uuid: Option<Uuid>,
    context: &mut Context,
    control: &Control,
) -> Result<(), EmitterError> {
    events::error::emit(error, uuid, context, control)
        .await
        .map_err(EmitterError::Emitting)
}
