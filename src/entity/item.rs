use crate::uuid::Uuid;

pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
