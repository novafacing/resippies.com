use std::fmt::Display;

use uuid::Uuid as ExtUuid;

#[derive(Debug, Clone, Default)]
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

fn main() {
    let uuid = Uuid::now_v7();
    println!("{}", uuid);
}