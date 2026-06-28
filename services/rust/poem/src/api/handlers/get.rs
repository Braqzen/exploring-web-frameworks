use crate::{state::State as ServerState, task::Task};
use poem::{
    http::StatusCode,
    web::{Data, Json, Path},
};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(name = "get", skip_all)]
#[poem::handler]
pub async fn fetch(
    Data(state): Data<&Arc<Mutex<ServerState>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, StatusCode> {
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
            return Err(StatusCode::NOT_FOUND);
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
