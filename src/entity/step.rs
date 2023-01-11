use crate::uuid::Uuid;

pub struct Step {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
