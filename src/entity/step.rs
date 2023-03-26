use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Decode, Encode, FromRow};

use crate::{db::connection, uuid::Uuid};

#[derive(FromRow, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Step {
    pub id: Uuid,
    pub name: String,
    pub description: String,
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

    pub async fn insert(step: &Step) -> Result<()> {
        let mut conn = connection().await?;
        sqlx::query(Step::QUERY_INSERT_STEP)
            .bind(&step.id)
            .bind(&step.name)
            .bind(&step.description)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}
