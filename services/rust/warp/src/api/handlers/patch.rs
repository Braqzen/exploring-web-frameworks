use crate::state::State as ServerState;
use serde_json::Value;
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

#[instrument(name = "patch", skip_all)]
pub async fn partial_update(
    id: Uuid,
    state: Arc<Mutex<ServerState>>,
    request: Value,
) -> Result<Response, Infallible> {
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

                    return Ok(json(&task).into_response());
                }
            }
            return Ok(with_status(reply(), StatusCode::BAD_REQUEST).into_response());
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            return Ok(with_status(reply(), StatusCode::NOT_FOUND).into_response());
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    Ok(with_status(reply(), StatusCode::INTERNAL_SERVER_ERROR).into_response())
}
