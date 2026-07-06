use crate::routes::errors::AppError;
use axum::extract::{FromRequest, FromRequestParts, Json, Path};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, FromRequestParts)]
#[from_request(via(Path), rejection(AppError))]
pub struct AppPath(pub Uuid);

#[derive(Deserialize, FromRequest)]
#[from_request(via(Json), rejection(AppError))]
pub struct AppJson<T>(pub T);
