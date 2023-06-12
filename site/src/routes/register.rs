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
use tracing::{debug, info, warn};
use util::hash_password;

#[derive(Debug, Clone, Builder)]
pub struct RegisterData {
    auth: AuthRenderData,
    #[builder(setter(strip_option), default)]
    err: Option<String>,
    #[builder(setter(strip_option), default)]
    saved: Option<RegisterPostForm>,
}

impl RenderData for RegisterData {
    fn user(&self) -> Option<UserModel> {
        self.auth.user()
    }

    fn theme(&self) -> entities::theme::Theme {
        self.auth.user().map(|u| u.theme).unwrap_or_default()
    }
}

impl RegisterData {
    fn err(&self) -> Option<String> {
        self.err.clone()
    }

    fn saved(&self) -> Option<RegisterPostForm> {
        self.saved.clone()
    }
}

define! {
    Register<'a>(data: &'a RegisterData) {
        @Layout { data, head: new! {}, main: RegisterForm { data } }
    }
    RegisterForm<'a>(data: &'a RegisterData) {
        div
        #"register-form"
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
                    ."fa-book" {}
                }
                @if let Some(err) = data.err() {
                    div
                    ."text-sm"
                    ."text-red-500"
                    ."text-center" {
                        @err
                    }
                }
                form
                ."space-y-4"
                ."text-foreground-light-500"
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
                        [name = "username", type = "text", placeholder = "username", value = data.saved().map(|f| f.username).unwrap_or_default()] {
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
                        [name = "email", type = "email", placeholder = "you@example.com", value = data.saved().map(|f| f.email).unwrap_or_default()] {
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
                    div {
                        label
                        .block
                        ."mb-2"
                        ."text-sm"
                        ."font-medium"
                        [for = "confirm_password"] {
                            "Confirm password:"
                        }
                        input
                        #"confirm-password"
                        ."p-2.5"
                        .block
                        .rounded
                        ."w-full"
                        .border
                        ."border-foreground-light-500"
                        [name = "confirm_password", type = "password", placeholder = "••••••••••••"] {

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
                        "hx-post" = "/register",
                        "hx-trigger" = "click",
                        "hx-swap" = "outerHTML",
                        "hx-target" = "#register-form"
                    ] { "Register" }
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
        let data = RegisterDataBuilder::default()
            .auth(
                AuthRenderDataBuilder::default()
                    .user(auth.current_user.map(|u| u.display(false)))
                    .build()?,
            )
            .build()?;

        Ok(Html(Register { data: &data }.to_string()).into_response())
    }
}

#[derive(Clone, Deserialize)]
pub struct RegisterPostForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

impl Debug for RegisterPostForm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterPostForm")
            .field("username", &self.username)
            .field("email", &self.email)
            .finish_non_exhaustive()
    }
}

pub async fn post(
    auth: AuthSessionContex,
    State(state): State<AppState>,
    Form(register): Form<RegisterPostForm>,
) -> Result<impl IntoResponse, AppError> {
    debug!("Registering user {:?}", register);
    if register.password != register.confirm_password {
        let data = RegisterDataBuilder::default()
            .auth(
                AuthRenderDataBuilder::default()
                    .user(auth.current_user.map(|u| u.display(false)))
                    .build()?,
            )
            .err("Passwords do not match!".to_string())
            .saved(RegisterPostForm {
                username: register.username,
                email: register.email,
                password: "".to_string(),
                confirm_password: "".to_string(),
            })
            .build()?;

        Ok(Html(RegisterForm { data: &data }.to_string()).into_response())
    } else if let Some(user) = UserEntity::find()
        .filter(UserColumn::Username.eq(&register.username))
        .one(&state.db)
        .await?
    {
        warn!("Tried to register existing user {:?}", user);
        // A user with this username already exists
        let data = RegisterDataBuilder::default()
            .auth(
                AuthRenderDataBuilder::default()
                    .user(auth.current_user.map(|u| u.display(false)))
                    .build()?,
            )
            .err(format!(
                "The username '{}' has already been registered!",
                register.username
            ))
            .saved(RegisterPostForm {
                username: "".to_string(),
                ..register
            })
            .build()?;

        Ok(Html(RegisterForm { data: &data }.to_string()).into_response())
    } else {
        // Create the user
        info!("Creating user {}", register.username);
        let user = ActiveUserModel {
            username: Set(register.username),
            email: Set(register.email),
            password_hash: Set(hash_password(&register.password)?),
            ..Default::default()
        }
        .insert(&state.db)
        .await?;

        auth.login_user(user.id);

        Ok([("hx-redirect", "/index")].into_response())
    }
}
