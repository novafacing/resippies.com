use std::collections::HashMap;

use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    Form,
};
use axum_template::{Key, RenderHtml};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::{entity::identity::Identity, render::RenderEngine};

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

pub async fn post_register(Form(register): Form<RegisterForm>) -> impl IntoResponse {
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
