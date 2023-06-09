use axum_macros::FromRef;
use sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
