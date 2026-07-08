use crate::routes::{
    errors::AppError,
    extractors::{AppJson, AppPath},
    guards::ChaosGuard,
};
use app::{state::AppState, task::PatchedTask};
use rocket::{State, http::Status, patch, serde::json::Json};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};

#[patch("/<id>", data = "<body>")]
#[instrument(skip_all)]
pub async fn patch_handler(
    _guard: ChaosGuard,
    id: AppPath,
    state: &State<Arc<Mutex<AppState>>>,
    body: AppJson<PatchedTask>,
) -> (Status, Json<Value>) {
    let id = id.into_inner();

    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let previous_operation = task.operation.clone();
            task.operation = body.into_inner().operation;

            info!(
                %id,
                secret = task.secret.len(),
                from_operation = previous_operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PATCH",
                "Patched task"
            );

            return (Status::Ok, Json(json!(task.to_owned())));
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            return AppError::TaskNotFound.into_response();
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    AppError::Internal.into_response()
}
