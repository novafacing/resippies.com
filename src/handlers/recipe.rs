use std::sync::Arc;

use axum::{extract::Path, response::IntoResponse};
use axum_template::{Key, RenderHtml, TemplateEngine};
use futures::StreamExt;
use serde::Serialize;
use tracing::debug;

use crate::{
    auth::AuthCtx,
    entity::{
        cookbook::Cookbook, identity::Identity, ingredient::Ingredient, item::Item, recipe::Recipe,
        step::Step,
    },
    render::RenderEngine,
    uuid::Uuid,
};

#[derive(Serialize, Debug)]
struct RecipeResponseInner {
    recipe: Recipe,
    author: Option<Identity>,
    cookbooks: Vec<Cookbook>,
    steps: Vec<Step>,
    ingredients: Vec<(Ingredient, Option<Item>)>,
}

#[derive(Serialize, Debug)]
pub struct RecipeResponse {
    id: Uuid,
    response: Option<RecipeResponseInner>,
}

#[derive(Serialize, Debug)]
pub struct RecipeNotFoundResponse {
    recipe_id: Uuid,
}

pub async fn get_recipe(
    _auth: AuthCtx,
    engine: RenderEngine,
    recipe_id: Path<Uuid>,
) -> impl IntoResponse {
    if let Ok(Some(recipe)) = Recipe::query_by_id(&recipe_id).await {
        debug!("Loading recipe {}", recipe_id.0);
        let author = Identity::query_by_id(&recipe.author).await.unwrap();
        let cookbooks = Cookbook::query_by_recipe(&recipe_id)
            .await
            .unwrap_or(vec![]);

        debug!("Loaded cookbooks: {:?}", cookbooks);

        let steps = Step::query_by_recipe(&recipe_id).await.unwrap_or(vec![]);

        debug!("Loaded steps: {:?}", steps);

        let ingredients = Ingredient::query_by_recipe(&recipe_id)
            .await
            .unwrap_or(vec![])
            .into_iter()
            .map(|ingredient| async move {
                let item = Item::query_by_id(&ingredient.item).await.unwrap();
                (ingredient, item)
            })
            .collect::<futures::stream::FuturesUnordered<_>>()
            .collect::<Vec<_>>()
            .await;

        debug!("Loaded ingredients: {:?}", ingredients);

        let response = RecipeResponse {
            response: Some(RecipeResponseInner {
                recipe,
                author,
                cookbooks,
                steps,
                ingredients,
            }),
            id: recipe_id.0,
        };

        debug!("Rendering recipe {:#?}", response);

        RenderHtml("/recipe", engine, response)
    } else {
        let response = RecipeResponse {
            response: None,
            id: recipe_id.0,
        };

        RenderHtml("/recipe_not_found", engine, response)
    }
}
