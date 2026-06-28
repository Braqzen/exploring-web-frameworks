use crate::{state::State as ServerState, task::Task};
use rocket::{State, http::Status, patch, serde::json::Json};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(name = "patch", skip_all)]
#[patch("/<id>", data = "<request>")]
pub async fn partial_update(
    id: Uuid,
    state: &State<Arc<Mutex<ServerState>>>,
    request: Json<Value>,
) -> Result<Json<Task>, Status> {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            // Code assumes only operation is changed
            if let Some(operation) = request.get("operation").and_then(|v| v.as_str()) {
                if let Ok(operation) = operation.try_into() {
                    let previous_operation = task.operation.clone();
                    task.operation = operation;

                    info!(
                        %id,
                        secret = task.secret,
                        from_operation = previous_operation.to_string(),
                        to_operation = task.operation.to_string(),
                        method = "PATCH",
                        "Patched task"
                    );

                    return Ok(Json(task.to_owned()));
                }
            }
            return Err(Status::BadRequest);
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            return Err(Status::NotFound);
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    Err(Status::InternalServerError)
}
