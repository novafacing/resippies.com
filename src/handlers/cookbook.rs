// Do the exact same thing as recipe but for a cookbook

use std::collections::HashMap;

use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
    Form,
};
use axum_template::{Key, RenderHtml};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    auth::AuthCtx,
    entity::{cookbook::Cookbook, identity::Identity, recipe::Recipe},
    render::RenderEngine,
    uuid::Uuid,
};

#[derive(Serialize, Debug)]
struct CookbookResponseInner {
    cookbook: Cookbook,
    author: Option<Identity>,
    recipes: Vec<Recipe>,
    cookbooks: HashMap<Uuid, Vec<Cookbook>>,
    authors: HashMap<Uuid, Option<Identity>>,
}

#[derive(Serialize, Debug)]
pub struct CookbookResponse {
    auth: Option<Identity>,
    id: Uuid,
    response: Option<CookbookResponseInner>,
}

#[derive(Deserialize, Debug)]
pub struct CreateCookbookForm {
    pub name: String,
    pub description: String,
    pub contributors: Vec<String>,
    pub visibility: String,
}

#[derive(Serialize, Debug)]
pub struct CreateCookbookResponse {
    auth: Option<Identity>,
}

pub async fn get_cookbook(
    auth: AuthCtx,
    engine: RenderEngine,
    cookbook_id: Path<Uuid>,
) -> impl IntoResponse {
    if let Ok(Some(cookbook)) = Cookbook::query_by_id(&cookbook_id).await {
        // Check if this user is authorized to view this cookbook
        if !cookbook.can_view(&auth.current_user).await {
            return Redirect::to("/login").into_response();
        }

        debug!("Loading cookbook {}", cookbook_id.0);
        let author = Identity::query_by_id(&cookbook.author).await.unwrap();

        let recipes = Recipe::query_by_cookbook(&cookbook_id)
            .await
            .unwrap_or(vec![]);

        debug!("Loaded recipes: {:?}", recipes);

        // Mapping of recipe id to cookbooks
        let cookbooks = recipes
            .iter()
            .map(|recipe| recipe.id.clone())
            .collect::<Vec<Uuid>>()
            .into_iter()
            .map(|recipe| async move {
                let cookbooks = Cookbook::query_by_recipe(&recipe).await.unwrap_or(vec![]);
                (recipe, cookbooks)
            })
            .collect::<futures::stream::FuturesUnordered<_>>()
            .collect::<HashMap<_, _>>()
            .await;

        debug!("Loaded cookbooks: {:?}", cookbooks);

        // Mapping of recipe id to authors
        let authors = recipes
            .iter()
            .map(|recipe| (recipe.id.clone(), recipe.author.clone()))
            .collect::<Vec<(_, _)>>()
            .into_iter()
            .map(|(recipe_id, author_id)| async move {
                let author = Identity::query_by_id(&author_id).await.unwrap();
                (recipe_id, author)
            })
            .collect::<futures::stream::FuturesUnordered<_>>()
            .collect::<HashMap<_, _>>()
            .await;

        debug!("Loaded authors: {:?}", authors);

        let response = CookbookResponse {
            auth: auth.current_user,
            id: cookbook_id.0,
            response: Some(CookbookResponseInner {
                cookbook,
                author,
                recipes,
                cookbooks,
                authors,
            }),
        };

        debug!("Rendering cookbook {:?}", response);

        RenderHtml("/cookbook", engine, response).into_response()
    } else {
        Redirect::to("/not_found").into_response()
    }
}

pub async fn get_create_cookbook(
    auth: AuthCtx,
    engine: RenderEngine,
    Key(key): Key,
) -> impl IntoResponse {
    debug!("Loading create cookbook page");

    if auth.current_user.is_none() {
        Redirect::to("/login").into_response()
    } else {
        let response = CreateCookbookResponse {
            auth: auth.current_user,
        };

        RenderHtml(key, engine, response).into_response()
    }
}

pub async fn post_create_cookbook(
    auth: AuthCtx,
    Form(form): Form<CreateCookbookForm>,
) -> impl IntoResponse {
    debug!("Creating cookbook {}", form.name);

    let cookbook =
        Cookbook::from_create_cookbook_form(form, auth.current_user.expect("No user logged in").id)
            .await
            .expect("Error creating cookbook");

    debug!("Created cookbook {}", cookbook.name);

    cookbook.insert().await.expect("Error inserting cookbook");

    debug!("Inserted cookbook {}", cookbook.name);

    Redirect::to(&format!("/cookbook/{}", cookbook.id)).into_response()
}
