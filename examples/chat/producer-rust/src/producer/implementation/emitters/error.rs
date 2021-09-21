use super::{events, identification, producer::Control, Context, EmitterError, ProducerError};
use uuid::Uuid;

pub async fn emit<E: std::error::Error>(
    error: ProducerError<E>,
    uuid: Option<Uuid>,
    context: &mut Context,
    identification: Option<&mut identification::Identification>,
    control: &Control,
) -> Result<(), EmitterError> {
    events::error::emit(error, uuid, context, identification, control)
        .await
        .map_err(EmitterError::Emitting)
}
