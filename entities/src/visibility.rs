use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Visibility {
    #[sea_orm(num_value = 0)]
    Public,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Public
    }
}
