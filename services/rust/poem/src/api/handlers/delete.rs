use crate::state::State as ServerState;
use poem::{
    http::StatusCode,
    web::{Data, Path},
};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(name = "delete", skip_all)]
#[poem::handler]
pub async fn remove(
    Data(state): Data<&Arc<Mutex<ServerState>>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
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
            return Ok(StatusCode::NO_CONTENT);
        } else {
            drop(state);
            warn!(%id, method = "DELETE", "Task not found");
            return Err(StatusCode::NOT_FOUND);
        }
    }

    error!(%id, method = "DELETE", "Poisoned lock");

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
