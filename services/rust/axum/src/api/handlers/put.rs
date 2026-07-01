use crate::{
    api::errors::{internal_server_error, task_not_found},
    state::State as ServerState,
    task::Task,
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
pub async fn put_handler(
    State(state): State<Arc<Mutex<ServerState>>>,
    Extension(id): Extension<Uuid>,
    Extension(new_task): Extension<Task>,
) -> Response {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let previous_task = task.clone();
            *task = new_task.clone();

            info!(
                %id,
                from_secret = previous_task.secret.len(),
                to_secret = task.secret.len(),
                from_operation = previous_task.operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PUT",
                "Overwrote task"
            );

            return Json(task).into_response();
        } else {
            drop(state);
            warn!(%id, method = "PUT", "Task not found");
            return task_not_found();
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    internal_server_error()
}
