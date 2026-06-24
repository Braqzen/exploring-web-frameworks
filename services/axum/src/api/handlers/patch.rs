use crate::server::State as ServerState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[axum::debug_handler]
#[instrument(name = "patch", skip_all)]
pub async fn partial_update(
    State(state): State<Arc<Mutex<ServerState>>>,
    Path(id): Path<Uuid>,
    Json(request): Json<Value>,
) -> impl IntoResponse {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            // Code assumes only operation is changed
            if let Some(operation) = request.get("operation").and_then(|v| v.as_str()) {
                if let Ok(operation) = operation.try_into() {
                    let previous_operation = task.operation.clone();
                    task.operation = operation;

                    info!(
                        %id,
                        secret = task.secret,
                        from_operation = previous_operation.to_string(),
                        to_operation = task.operation.to_string(),
                        method = "PATCH",
                        "Patched task"
                    );

                    return Json(task).into_response();
                }
            }
            return StatusCode::BAD_REQUEST.into_response();
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            return StatusCode::NOT_FOUND.into_response();
        }
    }
    drop(state);

    error!(%id, method = "PATCH", "Poisoned lock");

    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
