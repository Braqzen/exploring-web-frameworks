use crate::state::State as ServerState;
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;
use warp::{
    http::StatusCode,
    reply::{Reply, Response, json, reply, with_status},
};

#[instrument(skip_all)]
pub async fn get_handler(id: Uuid, state: Arc<Mutex<ServerState>>) -> Result<Response, Infallible> {
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

            return Ok(json(&task).into_response());
        } else {
            drop(state);
            warn!(%id, method = "GET", "Task not found");
            return Ok(with_status(reply(), StatusCode::NOT_FOUND).into_response());
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    Ok(with_status(reply(), StatusCode::INTERNAL_SERVER_ERROR).into_response())
}
