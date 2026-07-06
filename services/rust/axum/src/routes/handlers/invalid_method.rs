use crate::routes::errors::AppError;
use axum::{response::IntoResponse, response::Response};
use tracing::instrument;

#[axum::debug_handler]
#[instrument(skip_all)]
pub async fn invalid_method_handler() -> Response {
    AppError::InvalidMethod.into_response()
}
