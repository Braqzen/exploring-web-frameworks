use crate::{
    api::errors::{internal_server_error, task_not_found},
    state::State as ServerState,
    task::PatchedTask,
};
use axum::{
    Json,
    extract::{Extension, State},
    response::{IntoResponse, Response},
};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[axum::debug_handler]
#[instrument(skip_all)]
pub async fn patch_handler(
    State(state): State<Arc<Mutex<ServerState>>>,
    Extension(id): Extension<Uuid>,
    Extension(patched_task): Extension<PatchedTask>,
) -> Response {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            // Code assumes only operation is changed
            let previous_operation = task.operation.clone();
            task.operation = patched_task.operation.clone();

            info!(
                %id,
                secret = task.secret.len(),
                from_operation = previous_operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PATCH",
                "Patched task"
            );

            return Json(task).into_response();
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            return task_not_found();
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    internal_server_error()
}
