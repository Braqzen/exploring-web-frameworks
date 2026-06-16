use axum::Json;
use serde_json::Value;

#[axum::debug_handler]
pub async fn request(Json(request): Json<Value>) -> Json<Value> {
    let data = request
        .get("data")
        .and_then(|v| v.as_str())
        .unwrap_or("Missing data");
    Json(Value::String(data.to_string()))
}
