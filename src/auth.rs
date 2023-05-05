use axum_login::{extractors::AuthContext, SqliteStore};

use crate::{entity::identity::Identity, uuid::Uuid};

pub type AuthCtx = AuthContext<Uuid, Identity, SqliteStore<Identity>>;
