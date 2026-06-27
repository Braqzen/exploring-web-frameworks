use crate::state::State as ServerState;
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;
use warp::{
    http::StatusCode,
    reply::{Reply, Response, reply, with_status},
};

#[instrument(name = "delete", skip_all)]
pub async fn remove(id: Uuid, state: Arc<Mutex<ServerState>>) -> Result<Response, Infallible> {
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
            return Ok(with_status(reply(), StatusCode::NO_CONTENT).into_response());
        } else {
            drop(state);
            warn!(%id, method = "DELETE", "Task not found");
            return Ok(with_status(reply(), StatusCode::NOT_FOUND).into_response());
        }
    }

    error!(%id, method = "DELETE", "Poisoned lock");

    Ok(with_status(reply(), StatusCode::INTERNAL_SERVER_ERROR).into_response())
}
