use crate::config::{
    ACQUIRE_TIMEOUT, CONNECT_TIMEOUT, IDLE_TIMEOUT, MAX_CONNECTIONS, MAX_LIFETIME, MIN_CONNECTIONS,
    SQLCIPHER_KEY, SQLX_LOGGING, SQLX_LOGGING_LEVEL,
};
use anyhow::{anyhow, Result};
use dotenvy_macro::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

const DATABASE_URL: &str = dotenv!("DATABASE_URL");

pub async fn connect() -> Result<DatabaseConnection> {
    let mut options = ConnectOptions::new(DATABASE_URL.to_owned());
    options
        .max_connections(MAX_CONNECTIONS)
        .min_connections(MIN_CONNECTIONS)
        .connect_timeout(CONNECT_TIMEOUT)
        .acquire_timeout(ACQUIRE_TIMEOUT)
        .idle_timeout(IDLE_TIMEOUT)
        .max_lifetime(MAX_LIFETIME)
        .sqlx_logging(SQLX_LOGGING)
        .sqlx_logging_level(SQLX_LOGGING_LEVEL)
        .sqlcipher_key(SQLCIPHER_KEY);

    Database::connect(options)
        .await
        .map_err(|e| anyhow!("Failed to connect to database: {}", e))
}
