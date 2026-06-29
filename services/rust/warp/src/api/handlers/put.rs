use crate::{
    api::errors::{internal_server_error, task_not_found},
    state::State as ServerState,
    task::Task,
};
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;
use warp::reply::{Reply, Response, json};

#[instrument(skip_all)]
pub async fn put_handler(
    id: Uuid,
    state: Arc<Mutex<ServerState>>,
    request: Task,
) -> Result<Response, Infallible> {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let previous_task = task.clone();
            *task = request.clone();

            info!(
                %id,
                from_secret = previous_task.secret,
                to_secret = task.secret,
                from_operation = previous_task.operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PUT",
                "Overwrote task"
            );

            return Ok(json(&task).into_response());
        } else {
            drop(state);
            warn!(%id, method = "PUT", "Task not found");
            return Ok(task_not_found());
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    Ok(internal_server_error())
}
