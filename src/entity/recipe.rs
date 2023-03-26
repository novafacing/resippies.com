use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Decode, Encode, FromRow};

use crate::{db::connection, uuid::Uuid};

#[derive(Debug, FromRow, Encode, Decode, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub author: Uuid,
    pub name: String,
    pub description: String,
}

impl Recipe {
    pub const TABLE_NAME: &str = "recipes";
    pub const QUERY_SELECT_RECIPE_BY_ID: &str = "SELECT * FROM recipes WHERE id = ?";
    pub const QUERY_SELECT_RECIPE_BY_AUTHOR: &str = "SELECT * FROM recipes WHERE author = ?";
    pub const QUERY_INSERT_RECIPE: &str = r#"
        INSERT INTO recipes
            (id, author, name, description)
        VALUES
            (?, ?, ?, ?);
        "#;
    pub const QUERY_PUBLIC_VISIBLE_RECIPES_LIMIT: &str =
        "SELECT * FROM recipes WHERE visibility = 'public' LIMIT ? OFFSET ? ORDER BY created_at DESC";
    pub async fn query_by_id(id: &Uuid) -> Result<Option<Recipe>> {
        let mut conn = connection().await?;
        let recipe: Option<Recipe> = query_as(Recipe::QUERY_SELECT_RECIPE_BY_ID)
            .bind(id)
            .fetch_one(&mut conn)
            .await
            .ok();

        Ok(recipe)
    }

    pub async fn query_by_author(author: &Uuid) -> Result<Vec<Recipe>> {
        let mut conn = connection().await?;
        let recipes = query_as(Recipe::QUERY_SELECT_RECIPE_BY_AUTHOR)
            .bind(author)
            .fetch_all(&mut conn)
            .await?;

        Ok(recipes)
    }

    pub async fn query_public_recipes(limit: u32, offset: u32) -> Result<Vec<Recipe>> {
        let mut conn = connection().await?;
        let recipes = query_as(Recipe::QUERY_PUBLIC_VISIBLE_RECIPES_LIMIT)
            .bind(limit)
            .bind(offset)
            .fetch_all(&mut conn)
            .await?;

        Ok(recipes)
    }
}
