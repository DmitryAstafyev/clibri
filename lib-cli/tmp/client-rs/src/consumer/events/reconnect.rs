use super::Context;

pub async fn handler(timeout: u64, context: &mut Context) -> bool {
    println!("Will reconnect in {}", timeout);
    true
}
