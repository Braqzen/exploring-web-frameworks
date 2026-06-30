use crate::{
    api::{
        errors::{internal_server_error, task_not_found},
        guard::Extract,
    },
    state::State as ServerState,
    task::PatchedTask,
};
use rocket::{State, http::Status, patch, serde::json::Json};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[patch("/<id>", data = "<request>")]
#[instrument(skip_all)]
pub async fn patch_handler(
    id: Uuid,
    state: &State<Arc<Mutex<ServerState>>>,
    request: Extract<PatchedTask>,
) -> (Status, Json<Value>) {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            // Code assumes only operation is changed
            let previous_operation = task.operation.clone();
            task.operation = request.into_inner().operation;

            info!(
                %id,
                secret = task.secret,
                from_operation = previous_operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PATCH",
                "Patched task"
            );

            return (Status::Ok, Json(json!(task.to_owned())));
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            return task_not_found();
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    internal_server_error()
}
