use super::{Consumer, ConsumerError, Context};

pub async fn handler<E: std::error::Error + Clone>(
    error: ConsumerError<E>,
    context: &mut Context,
    consumer: Consumer<E>,
) {
}
