use super::{producer::Control, Context};
use crate::stop;
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    let summary = context.summary();
    println!("{}", summary);
    let errors = summary.get_errors();
    if !errors.is_empty() {
        for error in errors {
            eprintln!("{}", error);
        }
        stop!("");
    }
    Ok(())
}
