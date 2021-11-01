use super::{Consumer, ConsumerError, Context};

pub async fn handler<E: std::error::Error>(
    error: ConsumerError<E>,
    context: &mut Context,
    consumer: &mut Consumer<E>,
) {
}
