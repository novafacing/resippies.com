use crate::secret::secret;
use anyhow::Result;
use axum::async_trait;
use axum_session::SessionSqlitePool;
use axum_session_auth::AuthSession;
use entities::user::Model as UserModel;
use entities::Id;
use sea_orm::DatabaseConnection;
use sqlx::SqlitePool;
use uuid::Uuid;

pub type AuthSessionContex = AuthSession<UserModel, Id, SessionSqlitePool, SqlitePool>;
