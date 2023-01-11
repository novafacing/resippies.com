use crate::uuid::Uuid;

pub struct Ingredient {
    pub id: Uuid,
    pub item: Uuid,
    pub quantity: i64,
    pub unit: String,
}
