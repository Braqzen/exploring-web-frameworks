use crate::state::State as ServerState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[axum::debug_handler]
#[instrument(name = "get", skip_all)]
pub async fn fetch(
    State(state): State<Arc<Mutex<ServerState>>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Ok(state) = state.lock() {
        if let Some(task) = state.tasks.get(&id).cloned() {
            drop(state);
            info!(
                %id,
                secret = task.secret,
                operation = task.operation.to_string(),
                method = "GET",
                "Retrieved task"
            );

            return Json(task).into_response();
        } else {
            drop(state);
            warn!(%id, method = "GET", "Task not found");
            return StatusCode::NOT_FOUND.into_response();
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
