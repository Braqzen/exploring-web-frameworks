use crate::{state::State as ServerState, task::Task};
use poem::{
    http::StatusCode,
    web::{Data, Json, Path},
};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[poem::handler]
#[instrument(skip_all)]
pub async fn patch_handler(
    Data(state): Data<&Arc<Mutex<ServerState>>>,
    Path(id): Path<Uuid>,
    Json(request): Json<Value>,
) -> Result<Json<Task>, StatusCode> {
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
            return Err(StatusCode::BAD_REQUEST);
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            return Err(StatusCode::NOT_FOUND);
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
