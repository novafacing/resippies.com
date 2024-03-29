use crate::{db::connection, handlers::cookbook::CreateCookbookForm, uuid::Uuid};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Decode, Encode, FromRow};
use tracing::debug;

use super::{identity::Identity, recipe::Recipe};

#[derive(FromRow, Debug, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct Cookbook {
    pub id: Uuid,
    pub author: Uuid,
    pub name: String,
    pub description: String,
    pub visibility: String,
}

impl PartialEq for Cookbook {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Cookbook {}

impl Cookbook {
    pub const TABLE_NAME: &str = "cookbooks";
    pub const QUERY_SELECT_COOKBOOK_BY_ID: &str = "SELECT * FROM cookbooks WHERE id = ?";
    pub const QUERY_SELECT_COOKBOOK_BY_AUTHOR: &str = "SELECT * FROM cookbooks WHERE author = ?";
    pub const QUERY_INSERT_COOKBOOK: &str = r#"
        INSERT INTO cookbooks
            (id, author, name, description, visibility)
        VALUES
            (?, ?, ?, ?, ?);
        "#;
    pub const QUERY_INSERT_COOKBOOKS_CONTRIBUTORS: &str = r#"
        INSERT INTO cookbooks_contributors
            (cookbook, contributor)
        VALUES
            (?, ?);
        "#;
    pub const QUERY_SELECT_COOKBOOKS_BY_RECIPE: &str = r#"
        SELECT * FROM cookbooks
        INNER JOIN cookbooks_recipes
        ON cookbooks_recipes.cookbook = cookbooks.id
        WHERE cookbooks_recipes.recipe = ?"#;
    pub const QUERY_SELECT_COOKBOOKS_CONTRIBUTORS: &str =
        "SELECT contributor FROM cookbooks_contributors WHERE cookbook = ?";

    pub const QUERY_PUBLIC_VISIBLE_COOKBOOKS_LIMIT: &str =
        "SELECT * FROM cookbooks WHERE visibility = 'public' ORDER BY created_at DESC LIMIT ? OFFSET ?";

    pub const QUERY_DELETE_RECIPE_FROM_COOKBOOK: &str =
        "DELETE FROM cookbooks_recipes WHERE cookbook = ? AND recipe = ?";

    pub const QUERY_INSERT_RECIPE_TO_COOKBOOK: &str =
        "INSERT INTO cookbooks_recipes (cookbook, recipe) VALUES (?, ?)";
}

impl Cookbook {
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
        debug!("Inserted cookbook: {:?}", self);
        sqlx::query(Cookbook::QUERY_INSERT_COOKBOOKS_CONTRIBUTORS)
            .bind(&self.id)
            .bind(&self.author)
            .execute(&mut conn)
            .await?;
        debug!("Inserted cookbook contributor: {:?}", self);

        Ok(())
    }

    pub async fn remove_recipe(&self, recipe: &Recipe) -> Result<()> {
        let mut conn = connection().await?;
        sqlx::query(Cookbook::QUERY_DELETE_RECIPE_FROM_COOKBOOK)
            .bind(&self.id)
            .bind(&recipe.id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn add_recipe(&self, recipe: &Recipe) -> Result<()> {
        let mut conn = connection().await?;
        sqlx::query(Cookbook::QUERY_INSERT_RECIPE_TO_COOKBOOK)
            .bind(&self.id)
            .bind(&recipe.id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}

impl Cookbook {
    pub async fn from_create_cookbook_form(form: CreateCookbookForm, author: Uuid) -> Result<Self> {
        let cookbook = Cookbook {
            id: Uuid::now_v7(),
            author,
            name: form.name,
            description: form.description,
            visibility: form.visibility,
        };

        Ok(cookbook)
    }
}

impl Cookbook {
    pub async fn can_view(&self, identity: &Option<Identity>) -> bool {
        if self.visibility == "public" {
            true
        } else if let Some(identity) = identity {
            let mut conn = connection().await.unwrap();
            let contributors: Vec<Uuid> = query_as(Cookbook::QUERY_SELECT_COOKBOOKS_CONTRIBUTORS)
                .bind(&self.id)
                .fetch_all(&mut conn)
                .await
                .unwrap();
            contributors
                .iter()
                .any(|contributor| *contributor == identity.id)
        } else {
            false
        }
    }

    pub async fn can_edit(&self, identity: &Option<Identity>) -> bool {
        if let Some(identity) = identity {
            let mut conn = connection().await.unwrap();
            let contributors: Vec<Uuid> = query_as(Cookbook::QUERY_SELECT_COOKBOOKS_CONTRIBUTORS)
                .bind(&self.id)
                .fetch_all(&mut conn)
                .await
                .unwrap();
            contributors
                .iter()
                .any(|contributor| *contributor == identity.id)
        } else {
            false
        }
    }
}
