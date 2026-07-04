use crate::{routes::errors::AppError, state::AppState};
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;
use warp::reply::{Reply, Response, json};

#[instrument(skip_all)]
pub async fn get_handler(id: Uuid, state: Arc<Mutex<AppState>>) -> Result<Response, Infallible> {
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

            return Ok(json(&task).into_response());
        } else {
            drop(state);
            warn!(%id, method = "GET", "Task not found");
            return Ok(AppError::TaskNotFound.into_response());
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    Ok(AppError::Internal.into_response())
}
