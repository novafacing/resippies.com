mod auth;
mod db;
mod entity;
mod handlers;
mod password;
mod pattern;
mod render;
mod static_files;
mod templates;
mod uuid;

#[macro_use]
extern crate lazy_static;

use axum::{
    response::{IntoResponse, Redirect},
    routing::get,
    Router, Server,
};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer, SqliteStore,
};
use axum_template::engine::Engine;
use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine as _};
use dotenvy_macro::dotenv;
use entity::identity::Identity;
use rand::{thread_rng, Rng};
use render::RenderState;
use sqlx::sqlite::SqlitePoolOptions;
use templates::init_templates;
use tower_http::trace::TraceLayer;
use tracing::debug;
use tracing_subscriber::{
    fmt::layer, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt,
    EnvFilter,
};

use crate::db::DATABASE_URL;

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
                .unwrap_or_else(|_| "resippies_com=debug,tower_http=debug,axum_login=debug".into()),
        )
        .with(layer())
        .init();

    let encoded_secret: &str = dotenv!("SECRET_KEY").trim();
    let secret = BASE64_STANDARD_NO_PAD
        .decode(encoded_secret)
        .expect("Failed to decode secret key");

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

    let pool = SqlitePoolOptions::new()
        .connect(DATABASE_URL)
        .await
        .expect("Failed to connect to database");

    // TODO: with_query is called with_table until 4.0.2, this will change to with_query at some point!
    // TODO: let user_store = SqliteStore::<Identity>::new(pool).with_query(QUERY_SELECT_IDENTITY_BY_ID);
    let user_store = SqliteStore::<Identity>::new(pool).with_table_name(Identity::TABLE_NAME);
    let auth_layer = AuthLayer::new(user_store, &secret);

    let tera = init_templates()
        .await
        .expect("Failed to initialize templates");

    let app = Router::new()
        .route("/", get(handlers::index::redirect_index))
        .route("/index", get(handlers::index::get_index))
        .route(
            "/login",
            get(handlers::login::get_login).post(handlers::login::post_login),
        )
        .route(
            "/register",
            get(handlers::register::get_register).post(handlers::register::post_register),
        )
        .route("/static/*path", get(static_files::get_static))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(RenderState {
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
