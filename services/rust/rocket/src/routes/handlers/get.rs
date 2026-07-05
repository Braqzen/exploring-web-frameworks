use crate::{
    routes::{errors::AppError, extractors::AppPath, guards::ChaosGuard},
    state::AppState,
};
use rocket::{State, get, http::Status, serde::json::Json};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};

#[get("/<id>")]
#[instrument(skip_all)]
pub async fn get_handler(
    _guard: ChaosGuard,
    id: AppPath,
    state: &State<Arc<Mutex<AppState>>>,
) -> (Status, Json<Value>) {
    let id = id.into_inner();

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

            return (Status::Ok, Json(json!(task)));
        } else {
            drop(state);
            warn!(%id, method = "GET", "Task not found");
            return AppError::TaskNotFound.into_response();
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    AppError::Internal.into_response()
}
