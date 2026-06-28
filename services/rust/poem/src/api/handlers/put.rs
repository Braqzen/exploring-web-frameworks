use crate::{state::State as ServerState, task::Task};
use poem::{
    http::StatusCode,
    web::{Data, Json, Path},
};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(name = "put", skip_all)]
#[poem::handler]
pub async fn overwrite(
    Data(state): Data<&Arc<Mutex<ServerState>>>,
    Path(id): Path<Uuid>,
    Json(request): Json<Task>,
) -> Result<Json<Task>, StatusCode> {
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

            return Ok(Json(task.to_owned()));
        } else {
            drop(state);
            warn!(%id, method = "PUT", "Task not found");
            return Err(StatusCode::NOT_FOUND);
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
