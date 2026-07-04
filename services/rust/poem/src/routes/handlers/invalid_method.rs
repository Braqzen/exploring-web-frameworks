use crate::routes::errors::AppError;
use poem::{
    Response,
    error::{MethodNotAllowedError, ResponseError},
};
use tracing::instrument;

#[instrument(skip_all)]
pub async fn invalid_method_handler(_: MethodNotAllowedError) -> Response {
    AppError::InvalidMethod.as_response()
}
