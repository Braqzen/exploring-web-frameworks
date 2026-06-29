use salvo::{Response, http::StatusCode, writing::Json};
use serde_json::json;

pub fn task_not_found(res: &mut Response) {
    res.stuff(
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Task not found"})),
    );
}

pub fn internal_server_error(res: &mut Response) {
    res.stuff(
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error": "Internal server error"})),
    );
}
