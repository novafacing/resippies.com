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
        ."flex-auto"
        ."md:w-1/2"
        ."items-center"
        ."justify-center"
        ."px-6"
        ."py-8"
        ."mx-auto"
        {
            div
            ."w-full"
            ."rounded"
            ."border"
            ."border-foreground-light-500"
            ."bg-background-light-100"
            ."p-6"
            ."space-y-4" {
                h1
                ."text-xl"
                ."font-semibold"
                ."leading-tight"
                ."tracing-tight"
                ."text-center" {
                    "Log In"
                }
                form
                ."space-y-4"
                [action="/login", method="post"] {
                    div {
                        label
                        .block
                        ."mb-2"
                        ."text-sm"
                        ."font-medium"
                        [for = "username"] {
                            "Username:"
                        }
                        input
                        ."p-2.5"
                        .block
                        .rounded
                        ."w-full"
                        .border
                        ."border-foreground-light-500"
                        #username
                        [name = "username", type = "text", placeholder = "username"] {

                        }
                    }
                    div {
                        label
                        .block
                        ."mb-2"
                        ."text-sm"
                        ."font-medium"
                        [for = "email"] {
                            "Email:"
                        }
                        input
                        ."p-2.5"
                        .block
                        .rounded
                        ."w-full"
                        .border
                        ."border-foreground-light-500"
                        #email
                        [name = "email", type = "email", placeholder = "you@example.com"] {

                        }
                    }
                    div {
                        label
                        .block
                        ."mb-2"
                        ."text-sm"
                        ."font-medium"
                        [for = "password"] {
                            "Password:"
                        }
                        input
                        ."p-2.5"
                        .block
                        .rounded
                        ."w-full"
                        .border
                        ."border-foreground-light-500"
                        #password
                        [name = "password", type = "password", placeholder = "••••••••••••"] {

                        }

                    }
                    button
                    ."w-full"
                    ."rounded"
                    ."border"
                    ."border-foreground-light-500"
                    ."font-medium"
                    ."text-center"
                    ."px-5"
                    ."py-2.5"
                    ."text-sm"
                    ."hover:bg-foreground-light-500"
                    ."hover:text-background-light-100"
                    [type = "submit"] { "Login" }
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
