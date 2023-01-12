use std::fmt::Display;

use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteTypeInfo, Decode, Encode, Sqlite, Type};
use uuid::Uuid as ExtUuid;

#[derive(Debug, Clone, Default, Decode, Encode, Serialize, Deserialize)]
pub struct Uuid(String);

impl Uuid {
    pub fn now_v7() -> Self {
        Uuid(ExtUuid::now_v7().to_string())
    }
}

impl TryFrom<Uuid> for ExtUuid {
    type Error = uuid::Error;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        ExtUuid::parse_str(&value.0)
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.0.fmt(f)
    }
}

impl Type<Sqlite> for Uuid {
    fn type_info() -> SqliteTypeInfo {
        String::type_info()
    }
}