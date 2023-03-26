use anyhow::Result;
use dotenvy_macro::dotenv;
use sqlx::{pool::PoolConnection, sqlite::SqlitePoolOptions, Sqlite};

pub const DATABASE_URL: &str = dotenv!("DATABASE_URL");

pub async fn connection() -> Result<PoolConnection<Sqlite>> {
    let pool = SqlitePoolOptions::new()
        .connect(DATABASE_URL)
        .await
        .expect("Failed to connect to database");

    let conn = pool.acquire().await?;

    Ok(conn)
}
