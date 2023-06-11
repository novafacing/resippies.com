use entities::user::ActiveModel as ActiveUserModel;
use sea_orm_migration::{prelude::*, sea_orm::ActiveModelTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let anonymous = ActiveUserModel::anonymous();
        anonymous.insert(db).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let anonymous = ActiveUserModel::anonymous();
        anonymous.delete(db).await?;
        Ok(())
    }
}
