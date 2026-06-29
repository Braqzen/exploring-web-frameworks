use poem::{IntoResponse, Response, http::StatusCode, web::Json};
use serde_json::json;

pub fn task_not_found() -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Task not found"})),
    )
        .into_response()
}

pub fn internal_server_error() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error": "Internal server error"})),
    )
        .into_response()
}
