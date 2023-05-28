use anyhow::Result;
use tracing::metadata::LevelFilter;
use tracing_subscriber::FmtSubscriber;

use resippies::db::connect;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = FmtSubscriber::builder()
        .with_max_level(LevelFilter::INFO)
        .finish();
    let db = connect().await?;

    Ok(())
}
