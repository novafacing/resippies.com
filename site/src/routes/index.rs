use std::collections::HashSet;

use crate::auth::AuthSessionContex;
use crate::pages::layout::Layout;
use crate::state::AppState;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;
use sea_orm::DatabaseConnection;
use tailwind_rs::CLIConfig;
use tailwind_rs::CssInlineMode;
use tailwind_rs::TailwindBuilder;
use tracing::debug;
use tracing::info;
use tracing::trace;

pub async fn get(// auth: AuthSessionContex,
) -> impl IntoResponse {
    trace!("index::get");
    let mut render_response = Layout {
        dark: false,
        head: markup::new! {
            title { "Hello, world!" }
        },
        main: markup::new! {},
    };

    Html(render_response.to_string())
}

pub async fn redirect_to() -> impl IntoResponse {
    trace!("index::redirect_to");
    Redirect::to("/index").into_response()
}
