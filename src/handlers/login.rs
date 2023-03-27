use axum::{
    response::{IntoResponse, Redirect},
    Form,
};
use axum_template::{Key, RenderHtml};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use crate::{auth::AuthCtx, entity::identity::Identity, render::RenderEngine};

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    auth: Option<Identity>,
}

pub async fn get_login(auth: AuthCtx, engine: RenderEngine, Key(key): Key) -> impl IntoResponse {
    match auth.current_user {
        Some(_) => {
            let response = LoginResponse {
                auth: auth.current_user,
            };
            RenderHtml(key, engine, response)
        }
        None => {
            let response = LoginResponse {
                auth: auth.current_user,
            };
            RenderHtml(key, engine, response)
        }
    }
}

pub async fn post_login(mut auth: AuthCtx, Form(login): Form<LoginForm>) -> impl IntoResponse {
    debug!("Trying to log in {}", login.username);

    match Identity::from_login_form(login).await {
        Ok(identity) => {
            debug!("Generated identity for {}", identity.username);

            match auth.login(&identity).await {
                Ok(_) => {
                    info!("Logged in user {}", identity.username);
                    Redirect::to("/").into_response()
                }
                Err(e) => {
                    error!("Error logging in: {}. Redirecting to login page.", e);
                    Redirect::to("/login").into_response()
                }
            }
        }
        Err(e) => {
            error!("Error logging in: {}. Redirecting to login page.", e);
            Redirect::to("/login").into_response()
        }
    }
}
