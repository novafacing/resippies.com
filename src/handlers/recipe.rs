use std::str::FromStr;

use axum::{
    extract::Path,
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
    ingredients: Vec<(Ingredient, Item)>,
}

#[derive(Serialize, Debug)]
pub struct RecipeResponse {
    auth: Option<Identity>,
    id: Uuid,
    response: Option<RecipeResponseInner>,
    cookbooks: Vec<Cookbook>,
}

#[derive(Deserialize, Debug)]
pub struct CreateRecipeForm {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub cookbook: Vec<String>,
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
                let item = Item::query_by_id(&ingredient.item).await.unwrap().unwrap();
                (ingredient, item)
            })
            .collect::<futures::stream::FuturesOrdered<_>>()
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
            cookbooks: Vec::new(),
        };

        debug!("Rendering recipe {:#?}", response);

        RenderHtml("/recipe", engine, response).into_response()
    } else {
        let response = RecipeResponse {
            auth: auth.current_user,
            response: None,
            id: recipe_id.0,
            cookbooks: Vec::new(),
        };

        RenderHtml("/not_found", engine, response).into_response()
    }
}

pub async fn get_create_recipe(auth: AuthCtx, engine: RenderEngine) -> impl IntoResponse {
    if auth.current_user.is_none() {
        return Redirect::to("/login").into_response();
    }

    // Get the list of the user's cookbooks
    let cookbooks = Cookbook::query_by_author(&auth.current_user.as_ref().unwrap().id)
        .await
        .unwrap_or(vec![]);

    let response = RecipeResponse {
        auth: auth.current_user,
        response: None,
        id: Uuid::now_v7(),
        cookbooks,
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

        for cookbook in form.cookbook.iter() {
            let cookbook = Cookbook::query_by_id(&Uuid::from_str(cookbook).unwrap())
                .await
                .unwrap()
                .unwrap();

            cookbook.add_recipe(&recipe).await.unwrap();
        }

        Redirect::to(format!("/recipe/{}", recipe.id).as_str()).into_response()
    }
}

pub async fn get_edit_recipe(
    auth: AuthCtx,
    recipe_id: Path<Uuid>,
    engine: RenderEngine,
) -> impl IntoResponse {
    let recipe = Recipe::query_by_id(&recipe_id).await.unwrap().unwrap();

    if !recipe.can_edit(&auth.current_user).await {
        Redirect::to("/").into_response()
    } else if let Some(current_user) = auth.current_user {
        let author = Identity::query_by_id(&recipe.author).await.unwrap();
        let cookbooks = Cookbook::query_by_recipe(&recipe_id)
            .await
            .unwrap_or(vec![]);

        let steps = Step::query_by_recipe(&recipe_id).await.unwrap_or(vec![]);

        let ingredients = Ingredient::query_by_recipe(&recipe_id)
            .await
            .unwrap_or(vec![])
            .into_iter()
            .map(|ingredient| async move {
                let item = Item::query_by_id(&ingredient.item).await.unwrap().unwrap();
                (ingredient, item)
            })
            .collect::<futures::stream::FuturesOrdered<_>>()
            .collect::<Vec<_>>()
            .await;

        // Also get the list of the user's cookbooks
        let author_cookbooks = Cookbook::query_by_author(&current_user.id)
            .await
            .unwrap_or(vec![]);

        let response = RecipeResponse {
            auth: Some(current_user.clone()),
            response: Some(RecipeResponseInner {
                recipe,
                author,
                cookbooks,
                steps,
                ingredients,
            }),
            id: recipe_id.0,
            cookbooks: author_cookbooks,
        };

        debug!("Rendering recipe {:#?}", response);

        RenderHtml("/edit_recipe", engine, response).into_response()
    } else {
        Redirect::to("/login").into_response()
    }
}

pub async fn post_edit_recipe(
    auth: AuthCtx,
    recipe_id: Path<Uuid>,
    // NOTE: We use axum_extra::Form because it supports multiple values for the same key
    // using serde_html_form
    Form(form): Form<CreateRecipeForm>,
) -> impl IntoResponse {
    if let Some(current_user) = auth.current_user.as_ref() {
        let recipe = Recipe::query_by_id(&recipe_id).await.unwrap().unwrap();
        let mut cookbooks = Cookbook::query_by_recipe(&recipe_id)
            .await
            .unwrap_or(vec![]);

        let mut steps = Step::query_by_recipe(&recipe_id).await.unwrap_or(vec![]);

        let mut ingredients = Ingredient::query_by_recipe(&recipe_id)
            .await
            .unwrap_or(vec![])
            .into_iter()
            .map(|ingredient| async move {
                let item = Item::query_by_id(&ingredient.item).await.unwrap().unwrap();
                (ingredient, item)
            })
            .collect::<futures::stream::FuturesOrdered<_>>()
            .collect::<Vec<_>>()
            .await;

        if recipe.author != current_user.id {
            Redirect::to("/").into_response()
        } else {
            let recipe = Recipe {
                id: recipe.id.clone(),
                author: recipe.author.clone(),
                name: form.name.clone(),
                description: form.description.clone(),
                visibility: form.visibility.clone(),
                created_at: recipe.created_at,
            };

            recipe.update().await.unwrap();

            for (i, (name, description)) in form
                .step_name
                .iter()
                .zip(form.step_description.iter())
                .enumerate()
            {
                if let Some(mut step) = steps.get_mut(i) {
                    step.name = name.to_string();
                    step.description = description.to_string();
                    step.update().await.unwrap();
                } else {
                    let step = Step {
                        name: name.to_string(),
                        description: description.to_string(),
                        ..Default::default()
                    };
                    step.insert(&recipe, i.try_into().unwrap()).await.unwrap();
                };
            }

            for (i, (name, (description, (quantity, unit)))) in form
                .ingredient_name
                .iter()
                .zip(
                    form.ingredient_description.iter().zip(
                        form.ingredient_quantity
                            .iter()
                            .zip(form.ingredient_unit.iter()),
                    ),
                )
                .enumerate()
            {
                if let Some(ingredient_item) = ingredients.get_mut(i) {
                    let (mut ingredient, mut item) = ingredient_item.clone();
                    item.name = name.to_string();
                    item.description = description.to_string();
                    item.update().await.unwrap();

                    ingredient.quantity = *quantity;
                    ingredient.unit = unit.to_string();
                    ingredient.update().await.unwrap();
                } else {
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
                };
            }

            // Add newly added cookbooks
            for cookbook_id in form.cookbook.iter().map(|id| Uuid::from_str(id).unwrap()) {
                // Add the recipe if it's not already in the cookbook
                if !cookbooks.iter().any(|c| c.id == cookbook_id) {
                    let cookbook = Cookbook::query_by_id(&cookbook_id).await.unwrap().unwrap();
                    cookbook.add_recipe(&recipe).await.unwrap();
                }
                // Remove the cookbook from the list of cookbooks to remove
                cookbooks.remove(cookbooks.iter().position(|c| c.id == cookbook_id).unwrap());
            }

            // Remove cookbooks that were removed
            for cookbook in cookbooks {
                cookbook.remove_recipe(&recipe).await.unwrap();
            }

            Redirect::to(format!("/recipe/{}", recipe.id).as_str()).into_response()
        }
    } else {
        Redirect::to("/login").into_response()
    }
}

pub async fn get_delete_recipe(
    auth: AuthCtx,
    recipe_id: Path<Uuid>,
    engine: RenderEngine,
) -> impl IntoResponse {
    let recipe = Recipe::query_by_id(&recipe_id).await.unwrap().unwrap();

    if recipe.can_edit(&auth.current_user).await {
        let author = Identity::query_by_id(&recipe.author).await.unwrap();
        let cookbooks = Cookbook::query_by_recipe(&recipe_id)
            .await
            .unwrap_or(vec![]);

        let steps = Step::query_by_recipe(&recipe_id).await.unwrap_or(vec![]);

        let ingredients = Ingredient::query_by_recipe(&recipe_id)
            .await
            .unwrap_or(vec![])
            .into_iter()
            .map(|ingredient| async move {
                let item = Item::query_by_id(&ingredient.item).await.unwrap().unwrap();
                (ingredient, item)
            })
            .collect::<futures::stream::FuturesOrdered<_>>()
            .collect::<Vec<_>>()
            .await;

        let response = RecipeResponse {
            auth: auth.current_user.clone(),
            response: Some(RecipeResponseInner {
                recipe,
                author,
                cookbooks,
                steps,
                ingredients,
            }),
            id: recipe_id.0,
            cookbooks: Vec::new(),
        };

        debug!("Rendering recipe {:#?}", response);

        RenderHtml("/delete_recipe", engine, response).into_response()
    } else {
        Redirect::to("/").into_response()
    }
}

pub async fn delete_delete_recipe(
    auth: AuthCtx,
    recipe_id: Path<Uuid>,
    engine: RenderEngine,
) -> impl IntoResponse {
    if let Some(current_user) = auth.current_user.as_ref() {
        let recipe = Recipe::query_by_id(&recipe_id).await.unwrap().unwrap();

        if recipe.author != current_user.id {
            Redirect::to("/").into_response()
        } else {
            recipe.delete().await.unwrap();

            let response = RecipeResponse {
                auth: Some(current_user.clone()),
                response: None,
                id: recipe_id.0,
                cookbooks: Vec::new(),
            };

            RenderHtml("/not_found", engine, response).into_response()
        }
    } else {
        Redirect::to("/login").into_response()
    }
}
