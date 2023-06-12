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
pub enum Theme {
    #[sea_orm(num_value = 0)]
    Light,
    #[sea_orm(num_value = 1)]
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}
