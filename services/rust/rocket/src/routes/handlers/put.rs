use crate::routes::{
    errors::AppError,
    extractors::{AppJson, AppPath},
    guards::ChaosGuard,
};
use app::{state::AppState, task::Task};
use rocket::{State, http::Status, put, serde::json::Json};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};

#[put("/<id>", data = "<body>")]
#[instrument(skip_all)]
pub async fn put_handler(
    _guard: ChaosGuard,
    id: AppPath,
    state: &State<Arc<Mutex<AppState>>>,
    body: AppJson<Task>,
) -> (Status, Json<Value>) {
    let id = id.into_inner();

    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let previous_task = task.clone();
            *task = body.into_inner();

            info!(
                %id,
                from_secret = previous_task.secret.len(),
                to_secret = task.secret.len(),
                from_operation = previous_task.operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PUT",
                "Overwrote task"
            );

            return (Status::Ok, Json(json!(task.to_owned())));
        } else {
            drop(state);
            warn!(%id, method = "PUT", "Task not found");
            return AppError::TaskNotFound.into_response();
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    AppError::Internal.into_response()
}
