use sea_orm::{entity::prelude::*, entity::*, DatabaseBackend, MockDatabase, Transaction};

#[tokio::test]
async fn test_get_user_passhash() -> Result<(), DbErr> {
    let db = MockDatabase::new(DatabaseBackend::Sqlite);
    Ok(())
}
