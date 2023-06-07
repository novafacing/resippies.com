use entities::{
    cookbook::{ActiveModel, Column, Model},
    prelude::Cookbook,
};
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ColumnTrait, DbConn, DbErr, DeleteResult, EntityTrait,
    QueryFilter, Set,
};

pub struct CookbookStore;

impl CookbookStore {
    pub async fn create_cookbook(db: &DbConn, cookbook: Model) -> Result<ActiveModel, DbErr> {
        ActiveModel {
            author: Set(cookbook.author),
            name: Set(cookbook.name),
            description: Set(cookbook.description),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_cookbook(db: &DbConn, id: Uuid, cookbook: Model) -> Result<Model, DbErr> {
        let existing: ActiveModel = Cookbook::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Cookbook with id {} not found",
                id
            )))
            .map(Into::into)?;

        ActiveModel {
            id: existing.id,
            author: existing.author,
            name: Set(cookbook.name),
            description: Set(cookbook.description),
            created_at: existing.created_at,
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_cookbook(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
        let cookbook: ActiveModel = Cookbook::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Cookbook with id {} not found",
                id
            )))
            .map(Into::into)?;

        cookbook.delete(db).await
    }

    pub async fn find_cookbook_by_id(db: &DbConn, id: Uuid) -> Result<Model, DbErr> {
        Cookbook::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Cookbook with id {} not found",
                id
            )))
    }

    pub async fn find_cookbooks_by_author(db: &DbConn, author: Uuid) -> Result<Vec<Model>, DbErr> {
        Cookbook::find()
            .filter(Column::Author.eq(author))
            .all(db)
            .await
    }
}
