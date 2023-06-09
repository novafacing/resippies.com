use std::collections::HashSet;

use crate::auth::AuthSessionContex;
use crate::layout::Layout;
use crate::state::AppState;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;
use sea_orm::DatabaseConnection;
use tailwind_rs::CLIConfig;
use tailwind_rs::CssInlineMode;
use tailwind_rs::TailwindBuilder;
use tracing::trace;

pub async fn get(// auth: AuthSessionContex,
) -> impl IntoResponse {
    trace!("index::get");
    let mut render_response = Layout {
        head: markup::new! {
            title { "Hello, world!" }
        },
        style: markup::new! {},
        main: markup::new! {},
    };

    let config = CLIConfig {
        minify: true,
        mode: CssInlineMode::Inline,
        obfuscate: false,

        ..Default::default()
    };
    let mut builder = config.builder();

    let (_html, css) = config
        .compile_html(&render_response.to_string(), &mut builder)
        .expect("Failed to compile HTML");

    render_response.style = markup::new! {
        style {
            @markup::raw(&css)
        }
    };

    Html(render_response.to_string())
}

pub async fn redirect_to() -> impl IntoResponse {
    trace!("index::redirect_to");
    Redirect::to("/index").into_response()
}