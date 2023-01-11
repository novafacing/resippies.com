use crate::uuid::Uuid;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Cookbook {
    pub id: Uuid,
    pub author: Uuid,
    pub name: String,
}
