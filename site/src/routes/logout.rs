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

#[derive(Debug, Clone, Builder)]
pub struct LogoutData {
    auth: AuthRenderData,
    #[builder(setter(strip_option), default)]
    err: Option<String>,
}

impl RenderData for LogoutData {
    fn user(&self) -> Option<UserModel> {
        self.auth.user()
    }

    fn theme(&self) -> entities::theme::Theme {
        self.auth.user().map(|u| u.theme).unwrap_or_default()
    }
}

impl LogoutData {
    fn err(&self) -> Option<String> {
        self.err.clone()
    }
}

define! {
    Logout<'a>(data: &'a LogoutData) {
        @Layout { data, head: new! {}, main: LogoutForm { data } }
    }
    LogoutForm<'a>(data: &'a LogoutData) {
        div
        #"logout-form"
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
                    "Log Out?"
                }
                @if let Some(err) = data.err() {
                    div
                    ."text-sm"
                    ."text-red-500"
                    ."text-center" {
                        @err
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
                    "hx-post" = "/logout",
                    "hx-trigger" = "click",
                    "hx-swap" = "outerHTML",
                    "hx-target" = "#logout-form"
                ] { "Log Out" }
            }
        }
    }
}

pub async fn get(
    auth: AuthSessionContex,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(_) = auth.current_user {
        let data = LogoutDataBuilder::default()
            .auth(
                AuthRenderDataBuilder::default()
                    .user(auth.current_user.map(|u| u.display(false)))
                    .build()?,
            )
            .build()?;

        Ok(Html(Logout { data: &data }.to_string()).into_response())
    } else {
        Ok(Redirect::to("/index").into_response())
    }
}

pub async fn post(
    auth: AuthSessionContex,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    auth.logout_user();
    Ok([("hx-redirect", "/index")].into_response())
}
