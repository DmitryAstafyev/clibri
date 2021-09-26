use super::{identification, producer::Control, Context};

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control,
) -> Result<(), String> {
    Ok(())
    // Err(String::from(
    //     "Event emitter \"connected\" isn't implemented",
    // ))
}
