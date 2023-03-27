use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
};
use axum_template::{Key, RenderHtml};
use serde::Serialize;
use tracing::debug;

use crate::{auth::AuthCtx, entity::identity::Identity, render::RenderEngine, uuid::Uuid};

#[derive(Serialize, Debug)]
pub struct IdentityResponse {
    auth: Option<Identity>,
    identity: Identity,
}

pub async fn get_identity(
    auth: AuthCtx,
    engine: RenderEngine,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    debug!("Loading identity page");

    if auth.current_user.is_none() {
        Redirect::to("/login").into_response()
    } else if id == auth.current_user.clone().unwrap().id {
        let response = IdentityResponse {
            auth: auth.current_user.clone(),
            identity: auth.current_user.unwrap(),
        };
        RenderHtml("/identity", engine, response).into_response()
    } else {
        let response = IdentityResponse {
            auth: auth.current_user,
            identity: Identity::query_by_id(&id).await.unwrap().unwrap(),
        };
        RenderHtml("/identity", engine, response).into_response()
    }
}
