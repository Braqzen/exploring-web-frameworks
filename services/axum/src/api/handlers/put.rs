use crate::{server::State as ServerState, task::Task};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::{Arc, Mutex};
use tracing::{error, info, warn};
use uuid::Uuid;

#[axum::debug_handler]
pub async fn overwrite(
    State(state): State<Arc<Mutex<ServerState>>>,
    Path(id): Path<Uuid>,
    Json(request): Json<Task>,
) -> impl IntoResponse {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let previous_task = task.clone();
            *task = request.clone();

            info!(
                %id,
                from_secret = previous_task.secret,
                to_secret = task.secret,
                from_operation = previous_task.operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PUT",
                "Overwrote task"
            );

            return Json(task).into_response();
        } else {
            drop(state);
            warn!(%id, method = "PUT", "Task not found");
            return StatusCode::NOT_FOUND.into_response();
        }
    }
    drop(state);

    error!(%id, method = "PUT", "Poisoned lock");

    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
