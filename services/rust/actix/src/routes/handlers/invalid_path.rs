use crate::routes::errors::AppError;
use actix_web::HttpResponse;

pub async fn invalid_path_handler() -> Result<HttpResponse, AppError> {
    Err(AppError::InvalidPath)
}
