use poem::{IntoResponse, Response, error::ResponseError, http::StatusCode, web::Json};
use serde_json::json;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AppError {
    TaskNotFound,
    Internal,
    InvalidPath,
    InvalidMethod,
    InvalidJsonBody,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::TaskNotFound => "Task not found",
            Self::Internal => "Internal server error",
            Self::InvalidPath => "Invalid path",
            Self::InvalidMethod => "Invalid method",
            Self::InvalidJsonBody => "Invalid body JSON",
        })
    }
}

impl Error for AppError {}

impl ResponseError for AppError {
    fn as_response(&self) -> Response
    where
        Self: Error + Send + Sync + 'static,
    {
        (self.status(), Json(json!({"error": self.to_string()}))).into_response()
    }

    fn status(&self) -> StatusCode {
        match self {
            Self::TaskNotFound | Self::InvalidPath => StatusCode::NOT_FOUND,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidMethod => StatusCode::METHOD_NOT_ALLOWED,
            Self::InvalidJsonBody => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}
