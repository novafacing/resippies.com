//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cookbook_recipe")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub cookbook: String,
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub recipe: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cookbook::Entity",
        from = "Column::Cookbook",
        to = "super::cookbook::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Cookbook,
    #[sea_orm(
        belongs_to = "super::recipe::Entity",
        from = "Column::Recipe",
        to = "super::recipe::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Recipe,
}

impl Related<super::cookbook::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cookbook.def()
    }
}

impl Related<super::recipe::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recipe.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
