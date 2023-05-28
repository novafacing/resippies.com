use dotenvy_macro::dotenv;
use std::time::Duration;
use tracing::log::LevelFilter;

pub const MAX_CONNECTIONS: u32 = 100;
pub const MIN_CONNECTIONS: u32 = 10;
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(15);
pub const IDLE_TIMEOUT: Duration = Duration::from_secs(3600);
pub const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(15);
pub const MAX_LIFETIME: Duration = Duration::from_secs(3600);
pub const SQLX_LOGGING: bool = true;
pub const SQLX_LOGGING_LEVEL: LevelFilter = LevelFilter::Info;
pub const SQLCIPHER_KEY: &str = dotenv!("SECRET_KEY");
