#[macro_use]
extern crate lazy_static;

use axum::{routing::get, Router, Server};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer, SqliteStore,
};
use axum_template::engine::Engine;
use entity::identity::Identity;
use rand::{thread_rng, Rng};
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::trace::TraceLayer;

mod db;
mod entity;
mod handlers;
mod password;
mod pattern;
mod render;
mod static_files;
mod templates;

use render::State;
use templates::init_templates;
use tracing::debug;
use tracing_subscriber::{
    fmt::layer, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt,
    EnvFilter,
};

#[cfg(debug_assertions)]
const ADDRESS: &str = "127.0.0.1";
#[cfg(debug_assertions)]
const PORT: u16 = 8000;

#[cfg(not(debug_assertions))]
const ADDRESS: &str = "0.0.0.0";

#[cfg(not(debug_assertions))]
const PORT: u16 = 80;

#[tokio::main]
async fn main() {
    registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "resippies_com=debug,tower_http=info".into()),
        )
        .with(layer())
        .init();

    let secret = thread_rng().gen::<[u8; 64]>();

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

    let pool = SqlitePoolOptions::new()
        .connect(db::USER_DB)
        .await
        .expect("Failed to connect to database");

    let user_store = SqliteStore::<Identity>::new(pool);
    let auth_layer = AuthLayer::new(user_store, &secret);

    let tera = init_templates()
        .await
        .expect("Failed to initialize templates");

    let app = Router::new()
        .route("/static/*path", get(static_files::get_static))
        .route(
            "/login",
            get(handlers::auth::get_login).post(handlers::auth::post_login),
        )
        .route(
            "/register",
            get(handlers::auth::get_register).post(handlers::auth::post_register),
        )
        .layer(auth_layer)
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(State {
            render_engine: Engine::from(tera),
        });

    debug!("Starting server on {}:{}", ADDRESS, PORT);

    Server::bind(
        &format!("{}:{}", ADDRESS, PORT)
            .parse()
            .expect("Failed to parse address"),
    )
    .serve(app.into_make_service())
    .await
    .expect("Failed to start server");
}
