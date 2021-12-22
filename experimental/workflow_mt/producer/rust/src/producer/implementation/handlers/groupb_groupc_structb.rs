use super::{
    broadcast, hub, identification, pack, producer::Control, protocol, responses, Context,
    HandlerError, ProducerError,
};
use clibri::server;
use uuid::Uuid;

pub async fn process<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: hub::filter::Filter,
    context: &Context,
    request: &protocol::GroupB::GroupC::StructB,
    sequence: u32,
    control: &Control<E, C>,
) -> Result<(), HandlerError> {
    let uuid = identification.uuid();
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let buffer = match responses::groupb_groupc_structb::response(
        identification,
        filter,
        context,
        request,
        control,
    )
    .await
    {
        Ok(conclusion) => match conclusion {
            responses::groupb_groupc_structb::Response::CaseB((
                mut response,
                mut broadcast_structd,
                mut broadcast_structf,
            )) => {
                broadcasting.push((
                    broadcast_structd.0,
                    pack(&0, &uuid, &mut broadcast_structd.1)?,
                ));
                broadcasting.push((
                    broadcast_structf.0,
                    pack(&0, &uuid, &mut broadcast_structf.1)?,
                ));
                pack(&sequence, &uuid, &mut response)?
            }
            responses::groupb_groupc_structb::Response::CaseC(mut response) => {
                pack(&sequence, &uuid, &mut response)?
            }
            responses::groupb_groupc_structb::Response::CaseD((
                mut response,
                mut broadcast_structj,
            )) => {
                broadcasting.push((
                    broadcast_structj.0,
                    pack(&0, &uuid, &mut broadcast_structj.1)?,
                ));
                pack(&sequence, &uuid, &mut response)?
            }
        },
        Err(mut error) => pack(&sequence, &uuid, &mut error)?,
    };
    control
        .send(buffer, Some(uuid))
        .await
        .map_err(|e: ProducerError<E>| HandlerError::Processing(e.to_string()))?;
    for msg in broadcasting.iter_mut() {
        broadcast::<E, C>(msg, control).await?;
    }
    Ok(())
}
