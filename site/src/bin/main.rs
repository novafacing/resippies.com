use anyhow::Result;
use axum::{
    routing::{get, get_service},
    Router, Server,
};
use axum_session::{SessionConfig, SessionLayer, SessionSqlitePool, SessionStore};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use entities::user::Model as UserModel;
use entities::Id;
use http::StatusCode;
use resippies::{
    config::{ADDRESS, PORT},
    db::connect,
    routes::*,
    secret::secret,
    state::AppState,
    templates::init_templates,
};
use sea_orm::ConnectionTrait;
use sqlx::SqlitePool;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{info, metadata::LevelFilter};
use tracing_subscriber::{
    fmt::layer, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt,
    EnvFilter, FmtSubscriber,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "tower_http=debug,axum_template=debug,tera=debug,sqlx=debug,
            axum_session=debug,axum_session_auth=debug,sea_orm=debug,sea_query=debug,
            service=trace,entities=trace,resippies=trace,util=trace"
                .into()
        }))
        .with(layer())
        .init();
    info!("Starting server");
    let db = connect().await?;
    let sessions_pool = db.get_sqlite_connection_pool().clone();

    // let session_store = DatabaseSessionStore::new(db.clone());
    let session_config = SessionConfig::default().with_table_name("sessions");
    let auth_config = AuthConfig::<Id>::default().set_cache(true);
    let session_store = SessionStore::<SessionSqlitePool>::new(
        Some(SessionSqlitePool::from(sessions_pool.clone())),
        session_config,
    );
    session_store.initiate().await?;

    let session_layer = SessionLayer::new(session_store);
    let auth_session_layer =
        AuthSessionLayer::<UserModel, Id, SessionSqlitePool, SqlitePool>::new(Some(sessions_pool))
            .with_config(auth_config);

    // let user_store = AuthStore::new(&db);
    // let user_store = AuthStore::new(&db);

    let tera = init_templates().await?;
    let state = AppState::new(tera, db);

    let app = Router::new()
        .route("/", get(index::redirect_to))
        .route("/index", get(index::get))
        // .layer(session_layer)
        // .layer(auth_session_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
        .nest_service(
            "/static",
            get_service(ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static"
            )))
            .handle_error(|e| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", e),
                )
            }),
        );

    Server::bind(&format!("{}:{}", ADDRESS, PORT).parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
