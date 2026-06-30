use crate::{
    api::errors::{internal_server_error, task_not_found},
    state::State as ServerState,
};
use rocket::{State, delete, http::Status, response::status::NoContent, serde::json::Json};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[delete("/<id>")]
#[instrument(skip_all)]
pub async fn delete_handler(
    id: Uuid,
    state: &State<Arc<Mutex<ServerState>>>,
) -> Result<NoContent, (Status, Json<Value>)> {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.remove(&id) {
            drop(state);
            info!(
                %id,
                secret = task.secret,
                operation = task.operation.to_string(),
                method = "DELETE",
                "Removed task"
            );
            return Ok(NoContent);
        } else {
            drop(state);
            warn!(%id, method = "DELETE", "Task not found");
            return Err(task_not_found());
        }
    }

    error!(%id, method = "DELETE", "Poisoned lock");

    Err(internal_server_error())
}
