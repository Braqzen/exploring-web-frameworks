use crate::{server::State as ServerState, task::Task};
use axum::{Json, extract::State};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tracing::info;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn insert(
    State(state): State<Arc<Mutex<ServerState>>>,
    Json(request): Json<Task>,
) -> Json<Value> {
    let id = Uuid::new_v4();

    {
        // Lazy unwrap, won't panic
        let mut state = state.lock().unwrap();
        state.tasks.insert(id, request.clone());
    }
    info!(
        %id,
        secret = request.secret,
        operation = request.operation.to_string(),
        "Inserted new task"
    );

    Json(Value::String(id.to_string()))
}
