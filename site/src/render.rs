use derive_builder::Builder;
use entities::{user::Model as UserModel, Id};

#[derive(Debug, Clone, Builder)]
pub struct AuthRenderData {
    pub user: Option<UserModel>,
}

impl AuthRenderData {
    pub fn user(&self) -> Option<UserModel> {
        self.user.clone()
    }
}
