use crate::auth::AuthSessionContex;
use crate::render::Empty;
use crate::render::RenderData;
use crate::render::RenderEngine;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum_template::{Key, Render, RenderHtml, TemplateEngine};
use sea_orm::DatabaseConnection;
use tracing::trace;

pub async fn get(
    // auth: AuthSessionContex,
    engine: RenderEngine,
    Key(key): Key,
) -> impl IntoResponse {
    trace!("index::get");
    let response = RenderData::<Empty>::default();
    trace!("index::get: response: {:?}", response);
    RenderHtml(key, engine, response)
}

pub async fn redirect_to() -> impl IntoResponse {
    trace!("index::redirect_to");
    Redirect::to("/index").into_response()
}
