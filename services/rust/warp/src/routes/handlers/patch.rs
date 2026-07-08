use crate::routes::errors::AppError;
use app::{state::AppState, task::PatchedTask};
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;
use warp::reply::{Reply, Response, json};

#[instrument(skip_all)]
pub async fn patch_handler(
    id: Uuid,
    state: Arc<Mutex<AppState>>,
    new_task: PatchedTask,
) -> Result<Response, Infallible> {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let previous_operation = task.operation.clone();
            task.operation = new_task.operation;

            info!(
                %id,
                secret = task.secret.len(),
                from_operation = previous_operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PATCH",
                "Patched task"
            );

            return Ok(json(&task).into_response());
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            return Ok(AppError::TaskNotFound.into_response());
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    Ok(AppError::Internal.into_response())
}
