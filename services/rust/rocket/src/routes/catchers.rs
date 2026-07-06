use crate::routes::{errors::AppError, extractors::error_json};
use rocket::{Request, catch, serde::json::Json};
use serde_json::Value;

#[catch(404)]
pub fn not_found(_: &Request) -> Json<Value> {
    AppError::InvalidPath.into_response().1
}

#[catch(405)]
pub fn method_not_allowed(_: &Request) -> Json<Value> {
    AppError::InvalidMethod.into_response().1
}

#[catch(500)]
pub fn internal_error(_: &Request) -> Json<Value> {
    AppError::Internal.into_response().1
}

#[catch(422)]
pub fn unprocessable(req: &Request) -> Json<Value> {
    error_json(req)
}
