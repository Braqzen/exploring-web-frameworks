use crate::routes::errors::AppError;
use poem::{
    Response,
    error::{NotFoundError, ResponseError},
};
use tracing::instrument;

#[instrument(skip_all)]
pub async fn invalid_path_handler(_: NotFoundError) -> Response {
    AppError::InvalidPath.as_response()
}
