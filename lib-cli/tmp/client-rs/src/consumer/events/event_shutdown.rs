use super::{Consumer, ConsumerError, Context};

pub async fn handler<E: std::error::Error>(
    error: Option<ConsumerError<E>>,
    context: &mut Context,
    consumer: &mut Consumer<E>,
) {
}
