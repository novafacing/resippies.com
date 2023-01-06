use axum_macros::FromRef;
use axum_template::engine::Engine;
use serde::Serialize;
use tera::Tera;

pub type RenderEngine = Engine<Tera>;

#[derive(Debug, Serialize)]
pub struct Empty;

#[derive(Clone, FromRef)]
pub struct State {
    pub render_engine: RenderEngine,
}
