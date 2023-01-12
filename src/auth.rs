use axum_login::{extractors::AuthContext, SqliteStore};

use crate::entity::identity::Identity;

pub type AuthCtx = AuthContext<Identity, SqliteStore<Identity>>;
