use serde_json::json;
use warp::{
    http::StatusCode,
    reply::{Reply, Response, json, with_status},
};

pub fn task_not_found() -> Response {
    with_status(
        json(&json!({ "error": "Task not found" })),
        StatusCode::NOT_FOUND,
    )
    .into_response()
}

pub fn internal_server_error() -> Response {
    with_status(
        json(&json!({ "error": "Internal server error" })),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
    .into_response()
}
