use axum::{
    body::{boxed, Empty, Full},
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use http::header::{HeaderValue, CONTENT_TYPE};
use include_dir::{include_dir, Dir};
use mime_guess::from_path;
use tracing::debug;

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

pub async fn get_static(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    debug!("Serving static file: {}", path);

    let mime_type = from_path(path).first_or_text_plain();

    match STATIC_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(boxed(Full::from(file.contents())))
            .unwrap(),
    }
}
