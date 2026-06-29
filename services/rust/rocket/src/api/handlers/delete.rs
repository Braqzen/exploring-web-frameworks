use crate::state::State as ServerState;
use rocket::{State, delete, http::Status, response::status::NoContent};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[delete("/<id>")]
#[instrument(skip_all)]
pub async fn delete_handler(
    id: Uuid,
    state: &State<Arc<Mutex<ServerState>>>,
) -> Result<NoContent, Status> {
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
            return Err(Status::NotFound);
        }
    }

    error!(%id, method = "DELETE", "Poisoned lock");

    Err(Status::InternalServerError)
}
