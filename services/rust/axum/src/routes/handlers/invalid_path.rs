use crate::routes::errors::AppError;
use axum::{response::IntoResponse, response::Response};
use tracing::instrument;

#[axum::debug_handler]
#[instrument(skip_all)]
pub async fn invalid_path_handler() -> Response {
    AppError::InvalidPath.into_response()
}
