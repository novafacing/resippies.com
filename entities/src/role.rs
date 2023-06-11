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
pub enum Role {
    #[sea_orm(num_value = 0)]
    User,
    #[sea_orm(num_value = 1)]
    Anonymous,
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}
