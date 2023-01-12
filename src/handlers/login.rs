use axum::{response::IntoResponse, Form};
use axum_template::{Key, RenderHtml};
use serde::{Deserialize, Serialize};

use crate::{auth::AuthCtx, render::RenderEngine};

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
