use actix_web::HttpResponse;
use serde_json::json;

pub fn internal_server_error() -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({ "error": "Internal server error" }))
}

pub fn task_not_found() -> HttpResponse {
    HttpResponse::NotFound().json(json!({ "error": "Task not found" }))
}
