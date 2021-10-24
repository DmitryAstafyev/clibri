use super::{identification, producer::Control, Context, ProducerError};
use fiber::server;
use uuid::Uuid;
// TODO:
// identification already has uuid -> uuid can be removed;
// but filter should be added as optional

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    error: ProducerError<E>,
    uuid: Option<Uuid>,
    context: &mut Context,
    identification: Option<&mut identification::Identification>,
    control: &Control<E, C>,
) -> Result<(), String> {
    Ok(())
}
