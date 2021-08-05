pub mod client;
pub mod stat;
pub mod config;
pub mod test_1;

use fiber::env::logs;

#[tokio::main]
async fn main() -> Result<(), String> {
    logs::init();
    test_1::Test::run().await?;
    Ok(())
}
