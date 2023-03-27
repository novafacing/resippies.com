use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{
    query_as,
    types::chrono::{DateTime, Utc},
    Decode, Encode, FromRow,
};

use crate::{db::connection, uuid::Uuid};

use super::identity::Identity;

#[derive(Debug, FromRow, Encode, Decode, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub author: Uuid,
    pub name: String,
    pub description: String,
    pub visibility: String,
    pub created_at: DateTime<Utc>,
}

impl Recipe {
    pub const TABLE_NAME: &str = "recipes";
    pub const QUERY_SELECT_RECIPE_BY_ID: &str = "SELECT * FROM recipes WHERE id = ?";
    pub const QUERY_SELECT_RECIPE_BY_AUTHOR: &str = "SELECT * FROM recipes WHERE author = ?";
    pub const QUERY_INSERT_RECIPE: &str = r#"
        INSERT INTO recipes
            (id, author, name, description, visibility)
        VALUES
            (?, ?, ?, ?, ?);
        "#;
    pub const QUERY_INSERT_RECIPES_CONTRIBUTORS: &str = r#"
        INSERT INTO recipes_contributors
            (recipe, contributor)
        VALUES
            (?, ?);
        "#;
    pub const QUERY_SELECT_RECIPES_BY_COOKBOOK: &str = "SELECT * FROM recipes INNER JOIN cookbooks_recipes ON cookbooks_recipes.recipe = recipes.id WHERE cookbooks_recipes.cookbook = ?";
    pub const QUERY_SELECT_RECIPES_CONTRIBUTORS: &str =
        "SELECT * FROM recipes_contributors WHERE recipe = ?";

    pub const QUERY_PUBLIC_VISIBLE_RECIPES_LIMIT: &str =
        "SELECT * FROM recipes WHERE visibility = 'public' ORDER BY created_at DESC LIMIT ? OFFSET ?";
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

    pub async fn query_by_cookbook(cookbook: &Uuid) -> Result<Vec<Recipe>> {
        let mut conn = connection().await?;
        let recipes = query_as(Recipe::QUERY_SELECT_RECIPES_BY_COOKBOOK)
            .bind(cookbook)
            .fetch_all(&mut conn)
            .await?;

        Ok(recipes)
    }

    pub async fn insert(&self) -> Result<()> {
        let mut conn = connection().await?;
        sqlx::query(Recipe::QUERY_INSERT_RECIPE)
            .bind(&self.id)
            .bind(&self.author)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.visibility)
            .execute(&mut conn)
            .await?;
        sqlx::query(Recipe::QUERY_INSERT_RECIPES_CONTRIBUTORS)
            .bind(&self.id)
            .bind(&self.author)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}

impl Recipe {
    pub async fn can_view(&self, identity: &Option<Identity>) -> bool {
        if self.visibility == "public" {
            true
        } else if let Some(identity) = identity {
            // Check if the identity is a contributor to the recipe
            let mut conn = connection().await.unwrap();
            let contributors: Vec<Identity> = query_as(Recipe::QUERY_SELECT_RECIPES_CONTRIBUTORS)
                .bind(&self.id)
                .fetch_all(&mut conn)
                .await
                .unwrap();
            contributors
                .iter()
                .any(|contributor| contributor.id == identity.id)
        } else {
            false
        }
    }

    pub async fn can_edit(&self, identity: &Option<Identity>) -> bool {
        if let Some(identity) = identity {
            // Check if the identity is a contributor to the recipe
            let mut conn = connection().await.unwrap();
            let contributors: Vec<Identity> = query_as(Recipe::QUERY_SELECT_RECIPES_CONTRIBUTORS)
                .bind(&self.id)
                .fetch_all(&mut conn)
                .await
                .unwrap();
            contributors
                .iter()
                .any(|contributor| contributor.id == identity.id)
        } else {
            false
        }
    }
}
