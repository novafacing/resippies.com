use axum_macros::FromRef;
use sea_orm::DatabaseConnection;
use tera::Tera;

use crate::render::RenderEngine;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub engine: RenderEngine,
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(tera: Tera, db: DatabaseConnection) -> Self {
        Self {
            engine: RenderEngine::new(tera),
            db,
        }
    }
}
