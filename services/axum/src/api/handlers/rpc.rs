use axum::Json;
use serde_json::Value;

#[axum::debug_handler]
pub async fn request(Json(_request): Json<Value>) -> Json<Value> {
    Json(Value::String("Hello, world!".to_string()))
}
