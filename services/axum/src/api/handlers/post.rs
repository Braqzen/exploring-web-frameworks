use crate::{server::State as ServerState, task::Task};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument};
use uuid::Uuid;

#[axum::debug_handler]
#[instrument(name = "insert", skip_all)]
pub async fn insert(
    State(state): State<Arc<Mutex<ServerState>>>,
    Json(request): Json<Task>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();

    if let Ok(mut state) = state.lock() {
        state.tasks.insert(id, request.clone());
        drop(state);

        info!(
            %id,
            secret = request.secret,
            operation = request.operation.to_string(),
            method = "POST",
            "Inserted new task"
        );

        return Json(Value::String(id.to_string())).into_response();
    }

    error!(
        %id,
        secret = request.secret,
        operation = request.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
