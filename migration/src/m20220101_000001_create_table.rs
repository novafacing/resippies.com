use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    Code,
    Verified,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Recipe {
    Table,
    Id,
    Author,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Ingredient {
    Table,
    Id,
    Recipe,
    Name,
    Description,
    Quantity,
    Unit,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Step {
    Table,
    Id,
    Recipe,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Cookbook {
    Table,
    Id,
    Author,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum CookbookRecipe {
    Table,
    Cookbook,
    Recipe,
}

#[derive(Iden)]
pub enum CookbookContributor {
    Table,
    Cookbook,
    User,
}

#[derive(Iden)]
pub enum RecipeContributor {
    Table,
    Recipe,
    User,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let user = Table::create()
            .table(User::Table)
            .if_not_exists()
            .col(ColumnDef::new(User::Id).uuid().primary_key().not_null())
            .col(
                ColumnDef::new(User::Username)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(User::Email).string().not_null().unique_key())
            .col(ColumnDef::new(User::PasswordHash).string().not_null())
            .col(ColumnDef::new(User::Code).string().not_null())
            .col(
                ColumnDef::new(User::Verified)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(
                ColumnDef::new(User::CreatedAt)
                    .date_time()
                    .not_null()
                    .default("CURRENT_TIMESTAMP"),
            )
            .col(
                ColumnDef::new(User::UpdatedAt)
                    .date_time()
                    .not_null()
                    .default("CURRENT_TIMESTAMP"),
            )
            .to_owned();

        let recipe = Table::create()
            .table(Recipe::Table)
            .if_not_exists()
            .col(ColumnDef::new(Recipe::Id).uuid().primary_key().not_null())
            .col(ColumnDef::new(Recipe::Author).uuid().not_null())
            .col(ColumnDef::new(Recipe::Name).string().not_null())
            .col(ColumnDef::new(Recipe::Description).string().not_null())
            .col(
                ColumnDef::new(Recipe::CreatedAt)
                    .date_time()
                    .not_null()
                    .default("CURRENT_TIMESTAMP"),
            )
            .col(
                ColumnDef::new(Recipe::UpdatedAt)
                    .date_time()
                    .not_null()
                    .default("CURRENT_TIMESTAMP"),
            )
            .to_owned();

        let ingredient = Table::create()
            .table(Ingredient::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Ingredient::Id)
                    .uuid()
                    .primary_key()
                    .not_null(),
            )
            .col(ColumnDef::new(Ingredient::Recipe).uuid().not_null())
            .col(ColumnDef::new(Ingredient::Name).string().not_null())
            .col(ColumnDef::new(Ingredient::Description).string().not_null())
            .col(ColumnDef::new(Ingredient::Quantity).string().not_null())
            .col(ColumnDef::new(Ingredient::Unit).string().not_null())
            .col(
                ColumnDef::new(Ingredient::CreatedAt)
                    .date_time()
                    .not_null()
                    .default("CURRENT_TIMESTAMP"),
            )
            .col(
                ColumnDef::new(Ingredient::UpdatedAt)
                    .date_time()
                    .not_null()
                    .default("CURRENT_TIMESTAMP"),
            )
            .to_owned();

        let step = Table::create()
            .table(Step::Table)
            .if_not_exists()
            .col(ColumnDef::new(Step::Id).uuid().primary_key().not_null())
            .col(ColumnDef::new(Step::Recipe).uuid().not_null())
            .col(ColumnDef::new(Step::Name).string().not_null())
            .col(ColumnDef::new(Step::Description).string().not_null())
            .col(
                ColumnDef::new(Step::CreatedAt)
                    .date_time()
                    .not_null()
                    .default("CURRENT_TIMESTAMP"),
            )
            .col(
                ColumnDef::new(Step::UpdatedAt)
                    .date_time()
                    .not_null()
                    .default("CURRENT_TIMESTAMP"),
            )
            .to_owned();

        let cookbook = Table::create()
            .table(Cookbook::Table)
            .if_not_exists()
            .col(ColumnDef::new(Cookbook::Id).uuid().primary_key().not_null())
            .col(ColumnDef::new(Cookbook::Author).uuid().not_null())
            .col(ColumnDef::new(Cookbook::Name).string().not_null())
            .col(ColumnDef::new(Cookbook::Description).string().not_null())
            .col(
                ColumnDef::new(Cookbook::CreatedAt)
                    .date_time()
                    .not_null()
                    .default("CURRENT_TIMESTAMP"),
            )
            .col(
                ColumnDef::new(Cookbook::UpdatedAt)
                    .date_time()
                    .not_null()
                    .default("CURRENT_TIMESTAMP"),
            )
            .to_owned();

        let cookbook_recipe = Table::create()
            .table(CookbookRecipe::Table)
            .if_not_exists()
            .col(ColumnDef::new(CookbookRecipe::Cookbook).uuid().not_null())
            .col(ColumnDef::new(CookbookRecipe::Recipe).uuid().not_null())
            .to_owned();

        let cookbook_contributor = Table::create()
            .table(CookbookContributor::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CookbookContributor::Cookbook)
                    .uuid()
                    .not_null(),
            )
            .col(ColumnDef::new(CookbookContributor::User).uuid().not_null())
            .to_owned();

        let recipe_contributor = Table::create()
            .table(RecipeContributor::Table)
            .if_not_exists()
            .col(ColumnDef::new(RecipeContributor::Recipe).uuid().not_null())
            .col(ColumnDef::new(RecipeContributor::User).uuid().not_null())
            .to_owned();

        manager
            .create_table(user)
            .await
            .and(manager.create_table(recipe).await)
            .and(manager.create_table(ingredient).await)
            .and(manager.create_table(step).await)
            .and(manager.create_table(cookbook).await)
            .and(manager.create_table(cookbook_recipe).await)
            .and(manager.create_table(cookbook_contributor).await)
            .and(manager.create_table(recipe_contributor).await)
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
            .and(
                manager
                    .drop_table(Table::drop().table(Recipe::Table).to_owned())
                    .await,
            )
            .and(
                manager
                    .drop_table(Table::drop().table(Ingredient::Table).to_owned())
                    .await,
            )
            .and(
                manager
                    .drop_table(Table::drop().table(Step::Table).to_owned())
                    .await,
            )
            .and(
                manager
                    .drop_table(Table::drop().table(Cookbook::Table).to_owned())
                    .await,
            )
            .and(
                manager
                    .drop_table(Table::drop().table(CookbookRecipe::Table).to_owned())
                    .await,
            )
            .and(
                manager
                    .drop_table(Table::drop().table(CookbookContributor::Table).to_owned())
                    .await,
            )
            .and(
                manager
                    .drop_table(Table::drop().table(RecipeContributor::Table).to_owned())
                    .await,
            )
    }
}
