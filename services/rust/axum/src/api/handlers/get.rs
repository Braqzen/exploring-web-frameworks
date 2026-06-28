use crate::{
    api::errors::{internal_server_error, task_not_found},
    state::State as ServerState,
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
pub async fn get_handler(
    State(state): State<Arc<Mutex<ServerState>>>,
    Extension(id): Extension<Uuid>,
) -> Response {
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
            return task_not_found();
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    internal_server_error()
}
