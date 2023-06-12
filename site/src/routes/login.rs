use crate::auth::AuthSessionContex;
use crate::components::Layout;
use crate::error::AppError;
use crate::render::AuthRenderDataBuilder;
use crate::state::AppState;
use crate::{render::AuthRenderData, traits::RenderData};
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use derive_builder::Builder;
use entities::user::{
    ActiveModel as ActiveUserModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
use markup::{define, new};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use std::fmt::{Debug, Formatter};
use tracing::{info, warn};
use util::validate_password;

#[derive(Debug, Clone, Builder)]
pub struct IndexData {
    auth: AuthRenderData,
    #[builder(setter(strip_option), default)]
    err: Option<String>,
}

impl RenderData for IndexData {
    fn user(&self) -> Option<UserModel> {
        self.auth.user()
    }

    fn theme(&self) -> entities::theme::Theme {
        self.auth.user().map(|u| u.theme).unwrap_or_default()
    }
}

impl IndexData {
    fn err(&self) -> Option<String> {
        self.err.clone()
    }
}

define! {
    Login<'a, R>(data: &'a R) where R: RenderData {
        @Layout { data, head: new! {}, main: LoginForm { data } }
    }
    LoginForm<'a, R>(data: &'a R) where R: RenderData {
        div
        #"login-form"
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
                    div
                    ."text-foreground-light-500"
                    ."fa-3x"
                    ."fa-solid"
                    ."fa-kitchen-set" {}
                }
                form
                ."text-foreground-light-500"
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
                    [
                        "hx-post" = "/login",
                        "hx-trigger" = "click",
                        "hx-swap" = "outerHTML",
                        "hx-target" = "#login-form",
                    ] { "Log In" }
                }
            }
        }
    }
}

pub async fn get(
    auth: AuthSessionContex,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(_) = auth.current_user {
        Ok(Redirect::to("/index").into_response())
    } else {
        let data = IndexDataBuilder::default()
            .auth(
                AuthRenderDataBuilder::default()
                    .user(auth.current_user.map(|u| u.display(false)))
                    .build()?,
            )
            .build()?;

        Ok(Html(Login { data: &data }.to_string()).into_response())
    }
}

#[derive(Clone, Deserialize)]
pub struct LoginPostForm {
    pub username: String,
    pub password: String,
}

impl Debug for LoginPostForm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoginPostForm")
            .field("username", &self.username)
            .finish_non_exhaustive()
    }
}

pub async fn post(
    auth: AuthSessionContex,
    State(state): State<AppState>,
    Form(login): Form<LoginPostForm>,
) -> Result<impl IntoResponse, AppError> {
    info!("Trying to log in {:?}", login);

    if let Some(_) = auth.current_user {
        warn!("User already logged in");
        Ok([("hx-redirect", "/index")].into_response())
    } else {
        if let Some(user) = UserEntity::find()
            .filter(UserColumn::Username.eq(&login.username))
            .one(&state.db)
            .await?
        {
            if validate_password(&login.password, &user.password_hash) {
                info!("User logged in");
                auth.login_user(user.id);
                return Ok([("hx-redirect", "/index")].into_response());
            } else {
                warn!("Invalid password for user {}", user.username);
            }
        }

        warn!("Invalid username or password");

        let data = IndexDataBuilder::default()
            .auth(
                AuthRenderDataBuilder::default()
                    .user(auth.current_user.map(|u| u.display(false)))
                    .build()?,
            )
            .err("Invalid username or password".to_string())
            .build()?;

        Ok(Html(LoginForm { data: &data }.to_string()).into_response())
    }
}
