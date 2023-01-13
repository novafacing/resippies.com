use anyhow::Result;
use sqlx::{query, query_as};

use crate::{db::connection, entity::identity::Identity, uuid::Uuid};

pub async fn query_identity_id(id: &Uuid) -> Result<Option<Identity>> {
    let mut conn = connection().await?;
    let identity: Option<Identity> = query_as("SELECT * FROM identities WHERE id = ?")
        .bind(&id)
        .fetch_one(&mut conn)
        .await
        .ok();

    Ok(identity)
}

pub async fn query_identity_username(username: &str) -> Result<Option<Identity>> {
    let mut conn = connection().await?;
    let identity: Option<Identity> = query_as("SELECT * FROM identities WHERE username = ?")
        .bind(&username)
        .fetch_one(&mut conn)
        .await
        .ok();

    Ok(identity)
}

pub async fn insert_identity(identity: &Identity) -> Result<()> {
    let mut conn = connection().await?;

    query(
        r#"
        INSERT INTO identities
            (id, username, email, password_hash, code, verified)
        VALUES
            (?, ?, ?, ?, ?, ?);
        "#,
    )
    .bind(&identity.id)
    .bind(&identity.username)
    .bind(&identity.email)
    .bind(&identity.password_hash)
    .bind(&identity.code)
    .bind(&identity.verified)
    .execute(&mut conn)
    .await?;

    Ok(())
}
