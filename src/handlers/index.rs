use axum::{
    response::{IntoResponse, Redirect},
    Form,
};
use axum_template::{Key, RenderHtml};
use serde::Serialize;
use tracing::{debug, error, info};

use crate::{auth::AuthCtx, entity::identity::Identity, render::RenderEngine};

#[derive(Serialize, Debug)]
pub struct IndexResponse {}

pub async fn get_index(auth: AuthCtx, engine: RenderEngine, Key(key): Key) -> impl IntoResponse {
    debug!("Loading index page");
    let response = IndexResponse {};
    RenderHtml(key, engine, response)
}

pub async fn redirect_index() -> impl IntoResponse {
    Redirect::to("/index").into_response()
}
