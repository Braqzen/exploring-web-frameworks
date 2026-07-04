use axum::{
    Json,
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub enum AppError {
    TaskNotFound,
    Internal,
    InvalidPath,
    InvalidMethod,
    InvalidJsonBody,
}

impl From<JsonRejection> for AppError {
    fn from(value: JsonRejection) -> Self {
        // Note: could be more expressive with errors instead of 1 variant
        match value {
            JsonRejection::BytesRejection(_) => AppError::InvalidJsonBody,
            JsonRejection::JsonDataError(_) => AppError::InvalidJsonBody,
            JsonRejection::JsonSyntaxError(_) => AppError::InvalidJsonBody,
            JsonRejection::MissingJsonContentType(_) => AppError::InvalidJsonBody,
            _ => AppError::InvalidJsonBody,
        }
    }
}

impl From<PathRejection> for AppError {
    fn from(_: PathRejection) -> Self {
        // Note: could be more expressive with errors instead of 1 variant
        AppError::InvalidPath
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::TaskNotFound => (StatusCode::NOT_FOUND, "Task not found"),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Self::InvalidPath => (StatusCode::NOT_FOUND, "Invalid path"),
            Self::InvalidMethod => (StatusCode::METHOD_NOT_ALLOWED, "Invalid method"),
            Self::InvalidJsonBody => (StatusCode::UNPROCESSABLE_ENTITY, "Invalid body JSON"),
        };

        (status, Json(json!({"error": message}))).into_response()
    }
}
