use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::Form;
use axum_template::RenderHtml;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
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
    auth: Option<Identity>,
    id: Uuid,
    response: Option<RecipeResponseInner>,
}

#[derive(Deserialize, Debug)]
pub struct CreateRecipeForm {
    pub name: String,
    pub description: String,
    pub visibility: String,
    #[serde(rename = "ingredient-name")]
    pub ingredient_name: Vec<String>,
    #[serde(rename = "ingredient-description")]
    pub ingredient_description: Vec<String>,
    #[serde(rename = "ingredient-quantity")]
    pub ingredient_quantity: Vec<f64>,
    #[serde(rename = "ingredient-unit")]
    pub ingredient_unit: Vec<String>,
    #[serde(rename = "step-name")]
    pub step_name: Vec<String>,
    #[serde(rename = "step-description")]
    pub step_description: Vec<String>,
}

pub async fn get_recipe(
    auth: AuthCtx,
    engine: RenderEngine,
    recipe_id: Path<Uuid>,
) -> impl IntoResponse {
    if let Ok(Some(recipe)) = Recipe::query_by_id(&recipe_id).await {
        // Check if this user is authorized to view this recipe
        if !recipe.can_view(&auth.current_user).await {
            return Redirect::to("/login").into_response();
        }

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
            auth: auth.current_user,
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

        RenderHtml("/recipe", engine, response).into_response()
    } else {
        let response = RecipeResponse {
            auth: auth.current_user,
            response: None,
            id: recipe_id.0,
        };

        RenderHtml("/not_found", engine, response).into_response()
    }
}

pub async fn get_create_recipe(auth: AuthCtx, engine: RenderEngine) -> impl IntoResponse {
    if auth.current_user.is_none() {
        return Redirect::to("/login").into_response();
    }

    let response = RecipeResponse {
        auth: auth.current_user,
        response: None,
        id: Uuid::now_v7(),
    };

    RenderHtml("/create_recipe", engine, response).into_response()
}

pub async fn post_create_recipe(
    auth: AuthCtx,
    // NOTE: We use axum_extra::Form because it supports multiple values for the same key
    // using serde_html_form
    Form(form): Form<CreateRecipeForm>,
) -> impl IntoResponse {
    if auth.current_user.is_none() {
        Redirect::to("/login").into_response()
    } else {
        let recipe = Recipe {
            name: form.name.clone(),
            description: form.description.clone(),
            author: auth.current_user.unwrap().id,
            visibility: form.visibility.clone(),
            ..Default::default()
        };

        recipe.insert().await.unwrap();

        for (i, (name, description)) in form
            .step_name
            .iter()
            .zip(form.step_description.iter())
            .enumerate()
        {
            let step = Step {
                name: name.to_string(),
                description: description.to_string(),
                ..Default::default()
            };

            step.insert(&recipe, i.try_into().unwrap()).await.unwrap();
        }

        for (name, (description, (quantity, unit))) in form.ingredient_name.iter().zip(
            form.ingredient_description.iter().zip(
                form.ingredient_quantity
                    .iter()
                    .zip(form.ingredient_unit.iter()),
            ),
        ) {
            let item = Item {
                name: name.to_string(),
                description: description.to_string(),
                ..Default::default()
            };

            item.insert().await.unwrap();

            let ingredient = Ingredient {
                item: item.id,
                quantity: *quantity,
                unit: unit.to_string(),
                ..Default::default()
            };

            ingredient.insert(&recipe).await.unwrap();
        }

        Redirect::to(format!("/recipe/{}", recipe.id).as_str()).into_response()
    }
}
