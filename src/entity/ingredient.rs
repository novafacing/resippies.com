use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Decode, Encode, FromRow};

use crate::{db::connection, uuid::Uuid};

#[derive(FromRow, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Ingredient {
    pub id: Uuid,
    pub item: Uuid,
    pub quantity: f64,
    pub unit: String,
}

impl Ingredient {
    pub const TABLE_NAME: &str = "ingredients";
    pub const QUERY_SELECT_INGREDIENT_BY_ID: &str = "SELECT * FROM ingredients WHERE id = ?";
    pub const QUERY_SELECT_INGREDIENTS_BY_RECIPE: &str = r#"
        SELECT * FROM ingredients
        INNER JOIN recipes_ingredients
        ON ingredients.id = recipes_ingredients.ingredient
        WHERE recipes_ingredients.recipe = ?
    "#;

    pub const QUERY_INSERT_INGREDIENT: &str = r#"
        INSERT INTO ingredients
            (id, item, quantity, unit)
        VALUES
            (?, ?, ?, ?);
        "#;

    pub async fn query_by_id(id: &Uuid) -> Result<Option<Ingredient>> {
        let mut conn = connection().await?;
        let ingredient: Option<Ingredient> = query_as(Ingredient::QUERY_SELECT_INGREDIENT_BY_ID)
            .bind(id)
            .fetch_one(&mut conn)
            .await
            .ok();

        Ok(ingredient)
    }

    pub async fn query_by_recipe(recipe: &Uuid) -> Result<Vec<Ingredient>> {
        let mut conn = connection().await?;
        let ingredients = query_as(Ingredient::QUERY_SELECT_INGREDIENTS_BY_RECIPE)
            .bind(recipe)
            .fetch_all(&mut conn)
            .await
            .expect("Failed to fetch ingredients");

        Ok(ingredients)
    }
}
