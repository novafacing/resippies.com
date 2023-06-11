use crate::auth::AuthSessionContex;
use crate::components::Layout;
use crate::error::AppError;
use crate::render::AuthRenderDataBuilder;
use crate::state::AppState;
use crate::{render::AuthRenderData, traits::RenderData};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use derive_builder::Builder;
use entities::user::Model as UserModel;
use markup::{define, new};

#[derive(Debug, Clone, Builder)]
pub struct IndexData {
    auth: AuthRenderData,
}

impl RenderData for IndexData {
    fn user(&self) -> Option<UserModel> {
        self.auth.user()
    }
}

define! {
    Login<'a, R>(data: &'a R) where R: RenderData {
        @Layout { data, head: new! {}, main: LoginForm { data } }
    }
    LoginForm<'a, R>(data: &'a R) where R: RenderData {
        div
        .flex
        ."flex-col"
        ."items-center"
        ."justify-center"
        ."px-6"
        ."py-8"
        ."mx-auto"
        {
            div
            ."w-full"
            ."rounded-lg"
            .shadow
            ."border-foreground-light-500"
            ."p-6"
            ."space-y-4" {
                form[action="/login", method="post"] {
                    label[for = "username"] { "Username:" }
                    input #username[name = "username", type = "text", placeholder = "username"] {}
                    label[for = "password"] { "Password:" }
                    input #password[name = "password", type = "password", placeholder = "password"] {}
                    button["hx-post" = "/login", "hx-trigger"="click", "hx-target"="#login", "hx-swap"="outerHTML"] { "Login" }
                }
            }
        }
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

    Ok(Html(Login { data: &data }.to_string()))
}
