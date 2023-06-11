use anyhow::Error;
use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub struct AppError(Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}
