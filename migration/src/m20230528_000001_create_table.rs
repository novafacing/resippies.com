use entities::prelude::*;
use sea_orm_migration::{prelude::*, sea_orm::Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        let mut user = schema.create_table_from_entity(User);
        user.if_not_exists();
        let mut recipe = schema.create_table_from_entity(Recipe);
        recipe.if_not_exists();
        let mut ingredient = schema.create_table_from_entity(Ingredient);
        ingredient.if_not_exists();
        let mut step = schema.create_table_from_entity(Step);
        step.if_not_exists();
        let mut cookbook = schema.create_table_from_entity(Cookbook);
        cookbook.if_not_exists();
        let mut cookbook_recipe = schema.create_table_from_entity(CookbookRecipe);
        cookbook_recipe.if_not_exists();
        let mut cookbook_contributor = schema.create_table_from_entity(CookbookContributor);
        cookbook_contributor.if_not_exists();

        manager
            .create_table(user)
            .await
            .and(manager.create_table(recipe).await)
            .and(manager.create_table(ingredient).await)
            .and(manager.create_table(step).await)
            .and(manager.create_table(cookbook).await)
            .and(manager.create_table(cookbook_recipe).await)
            .and(manager.create_table(cookbook_contributor).await)
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::drop();
        table
            .table(User)
            .table(Recipe)
            .table(Ingredient)
            .table(Step)
            .table(Cookbook)
            .table(CookbookRecipe)
            .table(CookbookContributor);

        manager.drop_table(table).await
    }
}
