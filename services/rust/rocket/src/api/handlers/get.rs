use crate::{state::State as ServerState, task::Task};
use rocket::{State, get, http::Status, serde::json::Json};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(name = "get", skip_all)]
#[get("/<id>")]
pub async fn fetch(id: Uuid, state: &State<Arc<Mutex<ServerState>>>) -> Result<Json<Task>, Status> {
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

            return Ok(Json(task));
        } else {
            drop(state);
            warn!(%id, method = "GET", "Task not found");
            return Err(Status::NotFound);
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    Err(Status::InternalServerError)
}
