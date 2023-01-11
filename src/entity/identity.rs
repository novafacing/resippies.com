use std::fmt::Display;

use crate::{
    db::{insert_identity, query_identity_username},
    uuid::Uuid,
};
use anyhow::{anyhow, Context, Result};
use axum_login::{secrecy::SecretVec, AuthUser};
use serde::{Deserialize, Serialize};
use sqlx::{Encode, FromRow, Type};

use crate::{
    handlers::auth::RegisterForm,
    password::hash_password,
    pattern::{PASSWORD_PATTERN, USERNAME_PATTERN},
};

#[derive(Debug, Default, Clone, FromRow, Encode, Serialize, Deserialize, Type)]
pub struct Identity {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub code: Uuid,
    pub verified: bool,
}

impl AuthUser for Identity {
    fn get_id(&self) -> String {
        self.username.clone()
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into_bytes())
    }
}

impl Identity {
    pub async fn from_form(form: RegisterForm) -> Result<Self> {
        if form.password != form.confirm_password {
            Err(anyhow!("Passwords do not match!"))
        } else if !PASSWORD_PATTERN.is_match(&form.password) {
            Err(anyhow!(
                "Password does not contain 12+ characters from the specified set."
            ))
        } else if !USERNAME_PATTERN.is_match(&form.username) {
            Err(anyhow!(
                "Username is not a letter followed by letters and numbers."
            ))
        } else if query_identity_username(&form.username).await?.is_some() {
            Err(anyhow!("Username already exists."))
        } else {
            let password_hash =
                hash_password(&form.password).context("Could not hash password.")?;

            let identity = Identity::new(form.username, form.email, password_hash);

            insert_identity(&identity).await?;

            Ok(identity)
        }
    }

    pub fn new(username: String, email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            username,
            email,
            password_hash,
            code: Uuid::now_v7(),
            verified: false,
        }
    }
}
