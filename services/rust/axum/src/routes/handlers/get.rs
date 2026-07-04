use crate::{
    routes::{errors::AppError, extractors::AppPath},
    state::AppState,
};
use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};

#[axum::debug_handler]
#[instrument(skip_all)]
pub async fn get_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    AppPath(id): AppPath,
) -> Response {
    if let Ok(state) = state.lock() {
        if let Some(task) = state.tasks.get(&id).cloned() {
            drop(state);
            info!(
                %id,
                secret = task.secret.len(),
                operation = task.operation.to_string(),
                method = "GET",
                "Retrieved task"
            );

            return Json(task).into_response();
        } else {
            drop(state);
            warn!(%id, method = "GET", "Task not found");
            return AppError::TaskNotFound.into_response();
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    AppError::Internal.into_response()
}
