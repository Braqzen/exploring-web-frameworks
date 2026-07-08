use crate::routes::errors::AppError;
use app::{state::AppState, task::Task};
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;
use warp::reply::{Reply, Response, json};

#[instrument(skip_all)]
pub async fn put_handler(
    id: Uuid,
    state: Arc<Mutex<AppState>>,
    new_task: Task,
) -> Result<Response, Infallible> {
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

            return Ok(json(&task).into_response());
        } else {
            drop(state);
            warn!(%id, method = "PUT", "Task not found");
            return Ok(AppError::TaskNotFound.into_response());
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    Ok(AppError::Internal.into_response())
}
