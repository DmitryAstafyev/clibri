pub mod client;
pub mod config;
pub mod stat;
pub mod test_1;
pub mod test_2;
pub mod test_3;
pub mod test_4;

use fiber::env::logs;

#[tokio::main]
async fn main() -> Result<(), String> {
    logs::init();
    test_1::Test::run().await?;
    test_2::Test::run().await?;
    test_3::Test::run().await?;
    test_4::Test::run().await?;
    Ok(())
}
