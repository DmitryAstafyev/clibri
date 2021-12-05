use super::{identification, producer::Control, Context, ProducerError};
use crate::stat::Alias;
use clibri::server;
use uuid::Uuid;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    error: ProducerError<E>,
    uuid: Option<Uuid>,
    context: &mut Context,
    identification: Option<&mut identification::Identification>,
    control: &Control<E, C>,
) -> Result<(), String> {
    if let Some(uuid) = uuid {
        eprintln!("Consumer error: {:?}", error);
        context.inc_stat(uuid, Alias::Error);
    } else {
        eprintln!("Producer error: {:?}", error);
    }
    Ok(())
}
