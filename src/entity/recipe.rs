use crate::uuid::Uuid;

pub struct Recipe {
    pub id: Uuid,
    pub author: Uuid,
    pub name: String,
    pub description: String,
}
