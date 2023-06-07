use anyhow::Result;
use dotenvy_macro::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tracing::info;

const DATABASE_URL: &str = dotenv!("DATABASE_URL");

pub async fn connect() -> Result<DatabaseConnection> {
    info!("Connecting to database: {}", DATABASE_URL);
    let conn = Database::connect(DATABASE_URL).await?;
    info!("Running migrations");
    Migrator::up(&conn, None).await?;
    Ok(conn)
}
