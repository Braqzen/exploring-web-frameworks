use serde_json::json;
use warp::{
    http::StatusCode,
    reply::{Reply, Response, json, with_status},
};

#[derive(Debug, Clone)]
pub enum AppError {
    TaskNotFound,
    Internal,
    InvalidPath,
    InvalidMethod,
    InvalidJsonBody,
    InvalidBodySize,
}

impl warp::reject::Reject for AppError {}

impl AppError {
    pub fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::TaskNotFound => (StatusCode::NOT_FOUND, "Task not found"),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            Self::InvalidPath => (StatusCode::NOT_FOUND, "Invalid path"),
            Self::InvalidMethod => (StatusCode::METHOD_NOT_ALLOWED, "Invalid method"),
            Self::InvalidJsonBody => (StatusCode::UNPROCESSABLE_ENTITY, "Invalid body JSON"),
            Self::InvalidBodySize => (StatusCode::PAYLOAD_TOO_LARGE, "Invalid body size"),
        };
        with_status(json(&json!({ "error": message })), status).into_response()
    }
}
