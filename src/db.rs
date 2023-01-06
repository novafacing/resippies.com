use anyhow::Result;
use sqlx::{query_as, sqlite::SqlitePoolOptions};

use crate::entity::identity::Identity;

pub const USER_DB: &str = "sqlite::memory:";

pub async fn username_is_unique(username: &str) -> Result<bool> {
    let pool = SqlitePoolOptions::new()
        .connect(USER_DB)
        .await
        .expect("Failed to connect to database");

    let mut conn = pool.acquire().await?;

    let identity = query_as!(
        Identity,
        "SELECT * FROM identities WHERE username = ?",
        username,
    )
    .fetch_one(&mut conn)
    .await?;

    false
}
