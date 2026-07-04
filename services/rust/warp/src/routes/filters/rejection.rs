use crate::routes::errors::AppError;
use std::convert::Infallible;
use warp::{reject::Rejection, reply::Response};

pub async fn handle_rejection(err: Rejection) -> Result<Response, Infallible> {
    if let Some(app_error) = err.find::<AppError>() {
        return Ok(app_error.clone().into_response());
    }
    if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        return Ok(AppError::InvalidMethod.into_response());
    }
    if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        return Ok(AppError::InvalidBodySize.into_response());
    }
    Ok(AppError::InvalidPath.into_response())
}
