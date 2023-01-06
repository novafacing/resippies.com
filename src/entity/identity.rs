use anyhow::{anyhow, Result};
use axum_login::{secrecy::SecretVec, AuthUser};
use sqlx::{sqlite::SqlitePoolOptions, FromRow};

use crate::{
    handlers::auth::RegisterForm,
    password::hash_password,
    pattern::{PASSWORD_PATTERN, USERNAME_PATTERN},
};

#[derive(Debug, Default, Clone, FromRow)]
pub struct Identity {
    pub username: String,
    pub email: String,
    pub password_hash: String,
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
        } else if !username_is_unique(&form.username) {
            Err(anyhow!("Username already exists."))
        } else {
            let password_hash = hash_password(&form.password)?;
            Ok(Self {
                username: form.username,
                email: form.email,
                password_hash,
            })
        }
    }
}
