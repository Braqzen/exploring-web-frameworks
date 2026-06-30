use rocket::{http::Status, serde::json::Json};
use serde_json::{Value, json};

pub fn task_not_found() -> (Status, Json<Value>) {
    (Status::NotFound, Json(json!({"error": "Task not found"})))
}

pub fn internal_server_error() -> (Status, Json<Value>) {
    (
        Status::InternalServerError,
        Json(json!({"error": "Internal server error"})),
    )
}

pub fn invalid_path() -> (Status, Json<Value>) {
    (Status::NotFound, Json(json!({"error": "Invalid path"})))
}
