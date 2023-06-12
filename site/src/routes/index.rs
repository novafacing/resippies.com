use crate::auth::AuthSessionContex;
use crate::components::Layout;
use crate::error::AppError;
use crate::render::AuthRenderDataBuilder;
use crate::state::AppState;
use crate::{render::AuthRenderData, traits::RenderData};
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;
use derive_builder::Builder;
use entities::{theme::Theme, user::Model as UserModel};
use markup::{define, new};
use tracing::trace;

#[derive(Debug, Clone, Builder)]
pub struct IndexData {
    auth: AuthRenderData,
}

impl RenderData for IndexData {
    fn user(&self) -> Option<UserModel> {
        self.auth.user()
    }

    fn theme(&self) -> Theme {
        self.auth.user().map(|u| u.theme).unwrap_or_default()
    }
}

define! {
    Index<'a, R>(data: &'a R) where R: RenderData {
        @Layout { data, head: new! {}, main: new! {} }
    }
}

pub async fn get(
    auth: AuthSessionContex,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let data = IndexDataBuilder::default()
        .auth(
            AuthRenderDataBuilder::default()
                .user(auth.current_user.map(|u| u.display(false)))
                .build()?,
        )
        .build()?;

    Ok(Html(Index { data: &data }.to_string()))
}

pub async fn redirect_to() -> impl IntoResponse {
    trace!("index::redirect_to");
    Redirect::to("/index").into_response()
}
