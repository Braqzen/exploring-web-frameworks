use rocket::{
    Request,
    http::Status,
    response::{Responder, Result},
    serde::json::Json,
};
use serde_json::{Value, json};

#[derive(Debug, Clone)]
pub enum AppError {
    TaskNotFound,
    Internal,
    InvalidPath,
    InvalidMethod,
    InvalidJsonBody,
}

impl AppError {
    pub fn status(self) -> Status {
        match self {
            Self::TaskNotFound => Status::NotFound,
            Self::Internal => Status::InternalServerError,
            Self::InvalidPath => Status::NotFound,
            Self::InvalidMethod => Status::MethodNotAllowed,
            Self::InvalidJsonBody => Status::UnprocessableEntity,
        }
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::TaskNotFound => "Task not found",
            Self::Internal => "Internal server error",
            Self::InvalidPath => "Invalid path",
            Self::InvalidMethod => "Method not allowed",
            Self::InvalidJsonBody => "Invalid body JSON",
        }
    }

    pub fn into_response(self) -> (Status, Json<Value>) {
        (
            self.clone().status(),
            Json(json!({"error": self.message()})),
        )
    }
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        self.into_response().respond_to(req)
    }
}
