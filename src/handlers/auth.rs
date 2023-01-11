use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_login::{extractors::AuthContext, SqliteStore};
use axum_template::{engine::Engine, Key, RenderHtml};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, sqlite::SqlitePoolOptions};
use tracing::{debug, error, info};

use crate::{
    db::DB_PATH,
    entity::identity::Identity,
    render::{Empty, RenderEngine},
};

type AuthCtx = AuthContext<Identity, SqliteStore<Identity>>;

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {}

pub async fn get_login(engine: RenderEngine, Key(key): Key) -> impl IntoResponse {
    let response = LoginResponse {};
    RenderHtml(key, engine, response)
}

pub async fn post_login(mut auth: AuthCtx, Form(login): Form<LoginForm>) {
    // let pool = SqlitePoolOptions::new()
    //     .connect(USER_DB)
    //     .await
    //     .expect("Failed to connect to database");

    // let mut conn = pool.acquire().await.expect("Failed to acquire connection");
    dbg!(&login);
}

#[derive(Deserialize, Debug)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    pub error_message: Option<String>,
}

impl From<&HashMap<String, String>> for RegisterResponse {
    fn from(value: &HashMap<String, String>) -> Self {
        Self {
            error_message: match value.get("error_message") {
                Some(s) => Some(s.to_string()),
                None => None,
            },
        }
    }
}

pub async fn get_register(
    Query(params): Query<HashMap<String, String>>,
    engine: RenderEngine,
    Key(key): Key,
) -> impl IntoResponse {
    let response = RegisterResponse::from(&params);
    RenderHtml(key, engine, response)
}

pub async fn post_register(
    mut auth: AuthCtx,
    Form(register): Form<RegisterForm>,
) -> impl IntoResponse {
    match Identity::from_form(register).await {
        Ok(identity) => {
            debug!("Generated identity for {}", identity.username);
            match auth.login(&identity).await {
                Ok(_) => {
                    info!("Logged in user {}", identity.username);
                    Redirect::to("/").into_response()
                }
                Err(e) => {
                    error!("Error logging in: {}. Redirecting to login page.", e);
                    Redirect::to(&format!("/login?error_message={}", e.to_string())).into_response()
                }
            }
        }
        Err(e) => {
            error!("Error registering: {}. Redirecting to register page.", e);
            Redirect::to(&format!("/register?error_message={}", e.to_string())).into_response()
        }
    }
}
