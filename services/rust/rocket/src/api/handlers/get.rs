use crate::{
    api::errors::{internal_server_error, task_not_found},
    state::State as ServerState,
};
use rocket::{State, get, http::Status, serde::json::Json};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[get("/<id>")]
#[instrument(skip_all)]
pub async fn get_handler(
    id: Uuid,
    state: &State<Arc<Mutex<ServerState>>>,
) -> (Status, Json<Value>) {
    if let Ok(state) = state.lock() {
        if let Some(task) = state.tasks.get(&id).cloned() {
            drop(state);
            info!(
                %id,
                secret = task.secret,
                operation = task.operation.to_string(),
                method = "GET",
                "Retrieved task"
            );

            return (Status::Ok, Json(json!(task)));
        } else {
            drop(state);
            warn!(%id, method = "GET", "Task not found");
            return task_not_found();
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    internal_server_error()
}
