use axum_template::engine::Engine;
use entities::user::Model as UserModel;
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Empty;

impl Default for Empty {
    fn default() -> Self {
        Self
    }
}

pub type RenderEngine = Engine<Tera>;

#[derive(Debug, Serialize, Default)]
pub struct GlobalRenderData {
    pub user: Option<UserModel>,
    pub error: Option<String>,
}

impl GlobalRenderData {
    pub fn user(mut self, user: Option<UserModel>) -> Self {
        self.user = user;
        self
    }

    pub fn error(mut self, error: Option<String>) -> Self {
        self.error = error;
        self
    }
}

#[derive(Debug, Serialize, Default)]
pub struct RenderData<T>
where
    T: core::fmt::Debug + Serialize + Default,
{
    pub global: GlobalRenderData,
    pub local: T,
}
