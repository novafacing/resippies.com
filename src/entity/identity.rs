use crate::{
    db::connection,
    handlers::login::LoginForm,
    handlers::register::RegisterForm,
    password::hash_password,
    password::validate_password,
    pattern::{PASSWORD_PATTERN, USERNAME_PATTERN},
    uuid::Uuid,
};
use anyhow::{anyhow, Context, Result};
use axum_login::{secrecy::SecretVec, AuthUser};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use sqlx::{Encode, FromRow, Type};
use tracing::debug;

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
        // UUID is stored as a string in the database
        self.id.to_string()
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into_bytes())
    }
}

// Database constants and queries
impl Identity {
    pub const TABLE_NAME: &str = "identities";
    pub const QUERY_SELECT_IDENTITY_BY_ID: &str = "SELECT * FROM identities WHERE id = ?";
    pub const QUERY_SELECT_IDENTITY_BY_USERNAME: &str =
        "SELECT * FROM identities WHERE username = ?";
    pub const QUERY_INSERT_IDENTITY: &str = r#"
        INSERT INTO identities
            (id, username, email, password_hash, code, verified)
        VALUES
            (?, ?, ?, ?, ?, ?);
        "#;
    pub async fn query_by_id(id: &Uuid) -> Result<Option<Identity>> {
        let mut conn = connection().await?;
        let identity: Option<Identity> = query_as(Identity::QUERY_SELECT_IDENTITY_BY_ID)
            .bind(&id)
            .fetch_one(&mut conn)
            .await
            .ok();

        Ok(identity)
    }

    pub async fn query_by_username(username: &str) -> Result<Option<Identity>> {
        let mut conn = connection().await?;
        let identity: Option<Identity> = query_as(Identity::QUERY_SELECT_IDENTITY_BY_USERNAME)
            .bind(&username)
            .fetch_one(&mut conn)
            .await
            .ok();

        Ok(identity)
    }

    pub async fn insert(identity: &Identity) -> Result<()> {
        let mut conn = connection().await?;

        query(Identity::QUERY_INSERT_IDENTITY)
            .bind(&identity.id)
            .bind(&identity.username)
            .bind(&identity.email)
            .bind(&identity.password_hash)
            .bind(&identity.code)
            .bind(&identity.verified)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}

impl Identity {
    pub async fn from_register_form(form: RegisterForm) -> Result<Self> {
        debug!("Trying to create identity from register form: {:?}", form);
        if form.password != form.confirm_password {
            Err(anyhow!("Passwords do not match"))
        } else if !PASSWORD_PATTERN.is_match(&form.password) {
            Err(anyhow!(
                "Password does not contain 12+ characters from the specified set"
            ))
        } else if !USERNAME_PATTERN.is_match(&form.username) {
            Err(anyhow!(
                "Username is not a letter followed by letters and numbers"
            ))
        } else if Identity::query_by_username(&form.username).await?.is_some() {
            Err(anyhow!("Username already exists"))
        } else {
            let password_hash =
                hash_password(&form.password).context("Could not hash password.")?;

            let identity = Identity::new(form.username, form.email, password_hash);

            Identity::insert(&identity).await?;

            Ok(identity)
        }
    }

    pub async fn from_login_form(form: LoginForm) -> Result<Self> {
        debug!("Trying to create identity from login form: {:?}", form);
        if let Some(identity) = Identity::query_by_username(&form.username).await? {
            if validate_password(&form.password, &identity.password_hash) {
                Ok(identity)
            } else {
                Err(anyhow!("Incorrect password"))
            }
        } else {
            Err(anyhow!("Username does not exist"))
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
