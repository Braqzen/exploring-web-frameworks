use crate::server::State as ServerState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::{Arc, Mutex};
use tracing::{error, info, warn};
use uuid::Uuid;

#[axum::debug_handler]
pub async fn remove(
    State(state): State<Arc<Mutex<ServerState>>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.remove(&id) {
            drop(state);
            info!(
                %id,
                secret = task.secret,
                operation = task.operation.to_string(),
                method = "DELETE",
                "Removed task"
            );
            return StatusCode::NO_CONTENT.into_response();
        } else {
            drop(state);
            warn!(%id, method = "DELETE", "Task not found");
            return StatusCode::NOT_FOUND.into_response();
        }
    }
    drop(state);

    error!(%id, method = "DELETE", "Poisoned lock");

    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
