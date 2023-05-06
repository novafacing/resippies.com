use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Decode, Encode, FromRow};

use crate::{db::connection, uuid::Uuid};

use super::recipe::Recipe;

#[derive(FromRow, Debug, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct Step {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl Default for Step {
    fn default() -> Self {
        Step {
            id: Uuid::now_v7(),
            name: String::new(),
            description: String::new(),
        }
    }
}

impl Step {
    pub const TABLE_NAME: &str = "steps";
    pub const QUERY_SELECT_STEP_BY_ID: &str = "SELECT * FROM steps WHERE id = ?";
    pub const QUERY_SELECT_STEPS_BY_RECIPE: &str = r#"
        SELECT * FROM steps
        INNER JOIN recipes_steps
        ON steps.id = recipes_steps.step
        WHERE recipes_steps.recipe = ?
        ORDER BY recipes_steps.num
    "#;

    pub const QUERY_INSERT_STEP: &str = r#"
        INSERT INTO steps
            (id, name, description)
        VALUES
            (?, ?, ?);
        "#;
    pub const QUERY_INSERT_RECIPES_STEPS: &str = r#"
        INSERT INTO recipes_steps
            (recipe, step, num)
        VALUES
            (?, ?, ?);
        "#;
    pub const QUERY_UPDATE_STEP_BY_ID: &str = r#"
        UPDATE steps
        SET name = ?, description = ?
        WHERE id = ?;
        "#;

    // Query to remove all steps from a recipe and remove all the entries in recipes_steps for the recipe
    pub const QUERY_DELETE_STEPS_BY_RECIPE: &str = r#"
        DELETE FROM steps
        WHERE id IN (
            SELECT step FROM recipes_steps WHERE recipe = ?
        );
        DELETE FROM recipes_steps WHERE recipe = ?;
        "#;
}

impl Step {
    pub async fn query_by_id(id: &Uuid) -> Result<Option<Step>> {
        let mut conn = connection().await?;
        let step: Option<Step> = query_as(Step::QUERY_SELECT_STEP_BY_ID)
            .bind(id)
            .fetch_one(&mut conn)
            .await
            .ok();

        Ok(step)
    }

    pub async fn query_by_recipe(recipe: &Uuid) -> Result<Vec<Step>> {
        let mut conn = connection().await?;
        let steps = query_as(Step::QUERY_SELECT_STEPS_BY_RECIPE)
            .bind(recipe)
            .fetch_all(&mut conn)
            .await?;

        Ok(steps)
    }

    pub async fn insert(&self, recipe: &Recipe, number: u32) -> Result<()> {
        let mut conn = connection().await?;
        sqlx::query(Step::QUERY_INSERT_STEP)
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.description)
            .execute(&mut conn)
            .await?;
        sqlx::query(Step::QUERY_INSERT_RECIPES_STEPS)
            .bind(&recipe.id)
            .bind(&self.id)
            .bind(number)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn update(&self) -> Result<()> {
        let mut conn = connection().await?;
        sqlx::query(Step::QUERY_UPDATE_STEP_BY_ID)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn delete_by_recipe(recipe: &Recipe) -> Result<()> {
        let mut conn = connection().await?;
        sqlx::query(Step::QUERY_DELETE_STEPS_BY_RECIPE)
            .bind(&recipe.id)
            .bind(&recipe.id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}
