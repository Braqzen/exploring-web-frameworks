use crate::{
    api::errors::{internal_server_error, task_not_found},
    state::State as ServerState,
    task::PatchedTask,
};
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;
use warp::reply::{Reply, Response, json};

#[instrument(skip_all)]
pub async fn patch_handler(
    id: Uuid,
    state: Arc<Mutex<ServerState>>,
    request: PatchedTask,
) -> Result<Response, Infallible> {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            // Code assumes only operation is changed
            let previous_operation = task.operation.clone();
            task.operation = request.operation;

            info!(
                %id,
                secret = task.secret,
                from_operation = previous_operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PATCH",
                "Patched task"
            );

            return Ok(json(&task).into_response());
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            return Ok(task_not_found());
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    Ok(internal_server_error())
}
