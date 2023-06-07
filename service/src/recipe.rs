use entities::{
    cookbook_recipe,
    prelude::{CookbookRecipe, Recipe},
    recipe::{ActiveModel, Column, Model},
};
use futures::{stream::iter, StreamExt, TryStreamExt};
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ColumnTrait, DbConn, DbErr, DeleteResult, EntityTrait,
    QueryFilter, Set,
};

pub struct RecipeStore;

impl RecipeStore {
    pub async fn create_recipe(db: &DbConn, recipe: Model) -> Result<ActiveModel, DbErr> {
        ActiveModel {
            author: Set(recipe.author),
            name: Set(recipe.name),
            description: Set(recipe.description),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_recipe(db: &DbConn, id: Uuid, recipe: Model) -> Result<Model, DbErr> {
        let existing: ActiveModel = Recipe::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Recipe with id {} not found",
                id
            )))
            .map(Into::into)?;

        ActiveModel {
            id: existing.id,
            author: existing.author,
            name: Set(recipe.name),
            description: Set(recipe.description),
            created_at: existing.created_at,
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_recipe(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
        let recipe: ActiveModel = Recipe::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Recipe with id {} not found",
                id
            )))
            .map(Into::into)?;

        recipe.delete(db).await
    }

    pub async fn find_recipe_by_id(db: &DbConn, id: Uuid) -> Result<Model, DbErr> {
        Recipe::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Recipe with id {} not found",
                id
            )))
    }

    pub async fn find_recipes_by_author(db: &DbConn, author: Uuid) -> Result<Vec<Model>, DbErr> {
        Recipe::find()
            .filter(Column::Author.eq(author))
            .all(db)
            .await
    }

    pub async fn find_recipes_by_cookbook(
        db: &DbConn,
        cookbook: Uuid,
    ) -> Result<Vec<Model>, DbErr> {
        let recipe_ids = CookbookRecipe::find()
            .filter(cookbook_recipe::Column::Cookbook.eq(cookbook))
            .all(db)
            .await?;

        iter(recipe_ids)
            .then(|recipe_id| Self::find_recipe_by_id(db, recipe_id.recipe))
            .try_collect::<Vec<_>>()
            .await
    }
}
