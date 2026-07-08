use crate::routes::{errors::AppError, extractors::AppPath, guards::ChaosGuard};
use app::state::AppState;
use rocket::{State, delete, http::Status, response::status::NoContent, serde::json::Json};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};

#[delete("/<id>")]
#[instrument(skip_all)]
pub async fn delete_handler(
    _guard: ChaosGuard,
    state: &State<Arc<Mutex<AppState>>>,
    id: AppPath,
) -> Result<NoContent, (Status, Json<Value>)> {
    let id = id.into_inner();

    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.remove(&id) {
            drop(state);
            info!(
                %id,
                secret = task.secret.len(),
                operation = task.operation.to_string(),
                method = "DELETE",
                "Removed task"
            );
            return Ok(NoContent);
        } else {
            drop(state);
            warn!(%id, method = "DELETE", "Task not found");
            return Err(AppError::TaskNotFound.into_response());
        }
    }

    error!(%id, method = "DELETE", "Poisoned lock");

    Err(AppError::Internal.into_response())
}
