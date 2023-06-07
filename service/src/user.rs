use entities::{
    prelude::User,
    user::{ActiveModel, Column, Model},
};

use futures::{stream::iter, StreamExt, TryStreamExt};
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ColumnTrait, DbConn, DbErr, DeleteResult, EntityTrait,
    QueryFilter, Set,
};

pub struct UserStore;

impl UserStore {
    pub async fn create_user(db: &DbConn, user: Model) -> Result<ActiveModel, DbErr> {
        ActiveModel {
            username: Set(user.username),
            email: Set(user.email),
            password_hash: Set(user.password_hash),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_user(db: &DbConn, id: Uuid, user: Model) -> Result<Model, DbErr> {
        let existing: ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "User with id {} not found",
                id
            )))
            .map(Into::into)?;

        ActiveModel {
            id: existing.id,
            username: Set(user.username),
            email: Set(user.email),
            password_hash: Set(user.password_hash),
            created_at: existing.created_at,
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_user(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
        let user: ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "User with id {} not found",
                id
            )))
            .map(Into::into)?;

        user.delete(db).await
    }

    pub async fn find_user_by_id(db: &DbConn, id: Uuid) -> Result<Option<Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }
}
