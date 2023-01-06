use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    Form,
};
use axum_login::{extractors::AuthContext, SqliteStore};
use axum_template::{engine::Engine, Key, RenderHtml};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, sqlite::SqlitePoolOptions};
use tracing::error;

use crate::{
    db::USER_DB,
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

impl RegisterForm {}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {}

pub async fn get_register(engine: RenderEngine, Key(key): Key) -> impl IntoResponse {
    let response = RegisterResponse {};
    RenderHtml(key, engine, response)
}

pub async fn post_register(mut auth: AuthCtx, Form(register): Form<RegisterForm>) {
    // let pool = SqlitePoolOptions::new()
    //     .connect(USER_DB)
    //     .await
    //     .expect("Failed to connect to database");

    // let mut conn = pool.acquire().await.expect("Failed to acquire connection");
    match Identity::from_form(register) {
        Ok(identity) => {
            dbg!(&identity);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
