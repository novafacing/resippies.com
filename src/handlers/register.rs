use axum::{
    response::{IntoResponse, Redirect},
    Form,
};
use axum_template::{Key, RenderHtml};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::{auth::AuthCtx, entity::identity::Identity, render::RenderEngine};

#[derive(Deserialize, Debug)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    pub auth: Option<Identity>,
}

pub async fn get_register(auth: AuthCtx, engine: RenderEngine, Key(key): Key) -> impl IntoResponse {
    let response = RegisterResponse {
        auth: auth.current_user,
    };
    RenderHtml(key, engine, response)
}

pub async fn post_register(
    _auth: AuthCtx,
    Form(register): Form<RegisterForm>,
) -> impl IntoResponse {
    match Identity::from_register_form(register).await {
        Ok(identity) => {
            debug!("Generated identity for {}", identity.username);
            Redirect::to("/login").into_response()
        }
        Err(e) => {
            error!("Error registering: {}. Redirecting to register page.", e);
            Redirect::to("/register").into_response()
        }
    }
}
