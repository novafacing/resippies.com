use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Decode, Encode, FromRow};

use crate::{db::connection, uuid::Uuid};

#[derive(FromRow, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl Item {
    pub const TABLE_NAME: &str = "items";
    pub const QUERY_SELECT_ITEM_BY_ID: &str = "SELECT * FROM items WHERE id = ?";
    pub const QUERY_INSERT_ITEM: &str = r#"
        INSERT INTO items
            (id, name, description)
        VALUES
            (?, ?, ?);
        "#;

    pub async fn query_by_id(id: &Uuid) -> Result<Option<Item>> {
        let mut conn = connection().await?;
        let item: Option<Item> = query_as(Item::QUERY_SELECT_ITEM_BY_ID)
            .bind(id)
            .fetch_one(&mut conn)
            .await
            .ok();

        Ok(item)
    }

    pub async fn insert(item: &Item) -> Result<()> {
        let mut conn = connection().await?;
        sqlx::query(Item::QUERY_INSERT_ITEM)
            .bind(&item.id)
            .bind(&item.name)
            .bind(&item.description)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}
