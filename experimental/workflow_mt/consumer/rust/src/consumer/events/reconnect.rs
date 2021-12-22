use super::Context;

pub async fn handler(timeout: u64, context: &mut Context) -> bool {
    println!("handler for event reconnect isn't implemented");
    // Return true to confirm reconnection; false - to refuse
    true
}
