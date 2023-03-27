use std::collections::HashMap;

use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
};
use axum_template::{Key, RenderHtml};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    auth::AuthCtx,
    entity::{cookbook::Cookbook, identity::Identity, recipe::Recipe},
    render::RenderEngine,
    uuid::Uuid,
};

#[derive(Serialize, Debug)]
pub struct IndexResponse {
    auth: Option<Identity>,
    recipes: Vec<Recipe>,
    // Map recipe id to author
    authors: HashMap<Uuid, Option<Identity>>,
    // Map recipe id to cookbook
    cookbooks: HashMap<Uuid, Vec<Cookbook>>,
}

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u32>,
}

pub async fn get_index(
    auth: AuthCtx,
    engine: RenderEngine,
    Key(key): Key,
    pagination: Query<Pagination>,
) -> impl IntoResponse {
    debug!("Loading index page");

    let page = pagination.page.unwrap_or(0);

    debug!("Loading recipes for page {}", page);

    let recipes = Recipe::query_public_recipes(100, page * 100)
        .await
        .unwrap_or(vec![]);

    let authors = recipes
        .iter()
        .map(|recipe| (recipe.id.clone(), recipe.author.clone()))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(id, author)| async move {
            let author_identity = Identity::query_by_id(&author).await.unwrap();
            (id, author_identity)
        })
        .collect::<futures::stream::FuturesUnordered<_>>()
        .collect::<HashMap<_, _>>()
        .await;

    let cookbooks = recipes
        .iter()
        .map(|recipe| recipe.id.clone())
        .collect::<Vec<Uuid>>()
        .into_iter()
        .map(|recipe| async move {
            let cookbook = Cookbook::query_by_recipe(&recipe).await.unwrap();
            (recipe, cookbook)
        })
        .collect::<futures::stream::FuturesUnordered<_>>()
        .collect::<HashMap<_, _>>()
        .await;

    let response = IndexResponse {
        auth: auth.current_user,
        recipes,
        authors,
        cookbooks,
    };
    RenderHtml(key, engine, response)
}

pub async fn redirect_index() -> impl IntoResponse {
    Redirect::to("/index").into_response()
}
