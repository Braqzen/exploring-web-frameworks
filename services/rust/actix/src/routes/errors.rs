use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    TaskNotFound,
    Internal,
    InvalidPath,
    InvalidMethod,
    InvalidJsonBody,
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::TaskNotFound => "Task not found",
            Self::Internal => "Internal server error",
            Self::InvalidPath => "Invalid path",
            Self::InvalidMethod => "Invalid method",
            Self::InvalidJsonBody => "Invalid body JSON",
        })
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::TaskNotFound | Self::InvalidPath => StatusCode::NOT_FOUND,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidMethod => StatusCode::METHOD_NOT_ALLOWED,
            Self::InvalidJsonBody => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({ "error": self.to_string() }))
    }
}
