use crate::routes::{
    errors::AppError,
    extractors::{AppJson, AppPath},
};
use app::{state::AppState, task::PatchedTask};
use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response},
};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};

#[axum::debug_handler]
#[instrument(skip_all)]
pub async fn patch_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    AppPath(id): AppPath,
    AppJson(patched_task): AppJson<PatchedTask>,
) -> Response {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
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
            return AppError::TaskNotFound.into_response();
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    AppError::Internal.into_response()
}
