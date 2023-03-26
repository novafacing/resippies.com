use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
};
use axum_template::{Key, RenderHtml};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{auth::AuthCtx, entity::recipe::Recipe, render::RenderEngine};

#[derive(Serialize, Debug)]
pub struct IndexResponse {
    recipes: Vec<Recipe>,
}

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u32>,
}

pub async fn get_index(
    _auth: AuthCtx,
    engine: RenderEngine,
    Key(key): Key,
    pagination: Query<Pagination>,
) -> impl IntoResponse {
    debug!("Loading index page");

    let page = pagination.page.unwrap_or(0);

    let recipes = Recipe::query_public_recipes(100, page * 100)
        .await
        .unwrap_or(vec![]);

    let response = IndexResponse { recipes };
    RenderHtml(key, engine, response)
}

pub async fn redirect_index() -> impl IntoResponse {
    Redirect::to("/index").into_response()
}
