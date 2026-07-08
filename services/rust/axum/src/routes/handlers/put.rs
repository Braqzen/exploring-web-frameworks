use crate::routes::{
    errors::AppError,
    extractors::{AppJson, AppPath},
};
use app::{state::AppState, task::Task};
use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response},
};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};

#[axum::debug_handler]
#[instrument(skip_all)]
pub async fn put_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    AppPath(id): AppPath,
    AppJson(new_task): AppJson<Task>,
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
            return AppError::TaskNotFound.into_response();
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    AppError::Internal.into_response()
}
