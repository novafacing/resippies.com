use crate::{db::connection, uuid::Uuid};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Decode, Encode, FromRow};

#[derive(FromRow, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Cookbook {
    pub id: Uuid,
    pub author: Uuid,
    pub name: String,
    pub visibility: String,
}

impl Cookbook {
    pub const TABLE_NAME: &str = "cookbooks";
    pub const QUERY_SELECT_COOKBOOK_BY_ID: &str = "SELECT * FROM cookbooks WHERE id = ?";
    pub const QUERY_SELECT_COOKBOOK_BY_AUTHOR: &str = "SELECT * FROM cookbooks WHERE author = ?";
    pub const QUERY_INSERT_COOKBOOK: &str = r#"
        INSERT INTO cookbooks
            (id, author, name, visibility)
        VALUES
            (?, ?, ?, ?);
        "#;
    pub const QUERY_SELECT_COOKBOOKS_BY_RECIPE: &str = r#"
        SELECT * FROM cookbooks
        INNER JOIN cookbooks_recipes
        ON cookbooks_recipes.cookbook = cookbooks.id
        WHERE cookbooks_recipes.recipe = ?"#;

    pub const QUERY_PUBLIC_VISIBLE_COOKBOOKS_LIMIT: &str =
        "SELECT * FROM cookbooks WHERE visibility = 'public' ORDER BY created_at DESC LIMIT ? OFFSET ?";

    pub async fn query_by_id(id: &Uuid) -> Result<Option<Cookbook>> {
        let mut conn = connection().await?;
        let cookbook: Option<Cookbook> = query_as(Cookbook::QUERY_SELECT_COOKBOOK_BY_ID)
            .bind(id)
            .fetch_one(&mut conn)
            .await
            .ok();

        Ok(cookbook)
    }

    pub async fn query_by_author(author: &Uuid) -> Result<Vec<Cookbook>> {
        let mut conn = connection().await?;
        let cookbooks = query_as(Cookbook::QUERY_SELECT_COOKBOOK_BY_AUTHOR)
            .bind(author)
            .fetch_all(&mut conn)
            .await?;

        Ok(cookbooks)
    }

    pub async fn query_public_cookbooks(limit: u32, offset: u32) -> Result<Vec<Cookbook>> {
        let mut conn = connection().await?;
        let cookbooks = query_as(Cookbook::QUERY_PUBLIC_VISIBLE_COOKBOOKS_LIMIT)
            .bind(limit)
            .bind(offset)
            .fetch_all(&mut conn)
            .await?;

        Ok(cookbooks)
    }

    pub async fn query_by_recipe(recipe: &Uuid) -> Result<Vec<Cookbook>> {
        let mut conn = connection().await?;
        let cookbooks = query_as(Cookbook::QUERY_SELECT_COOKBOOKS_BY_RECIPE)
            .bind(recipe)
            .fetch_all(&mut conn)
            .await?;

        Ok(cookbooks)
    }

    pub async fn insert(&self) -> Result<()> {
        let mut conn = connection().await?;
        sqlx::query(Cookbook::QUERY_INSERT_COOKBOOK)
            .bind(&self.id)
            .bind(&self.author)
            .bind(&self.name)
            .bind(&self.visibility)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}
