pub use sea_orm_migration::prelude::*;

mod m20230528_000001_create_table;
mod m20230611_042651_anonymous_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230528_000001_create_table::Migration),
            Box::new(m20230611_042651_anonymous_user::Migration),
        ]
    }
}
