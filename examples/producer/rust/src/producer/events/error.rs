use super::{identification, producer::Control, Context, ProducerError};
use clibri::server;
use uuid::Uuid;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    error: ProducerError<E>,
    uuid: Option<Uuid>,
    context: &mut Context,
    identification: Option<&identification::Identification>,
    control: &Control<E, C>,
) -> Result<(), String> {
    Ok(())
}
