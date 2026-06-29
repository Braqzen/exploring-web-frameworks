use crate::{
    api::errors::{internal_server_error, task_not_found},
    state::State as ServerState,
};
use poem::{IntoResponse, Response, http::StatusCode, web::Data};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[poem::handler]
#[instrument(skip_all)]
pub async fn delete_handler(
    Data(state): Data<&Arc<Mutex<ServerState>>>,
    Data(id): Data<&Uuid>,
) -> Response {
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
            return StatusCode::NO_CONTENT.into_response();
        } else {
            drop(state);
            warn!(%id, method = "DELETE", "Task not found");
            return task_not_found();
        }
    }

    error!(%id, method = "DELETE", "Poisoned lock");

    internal_server_error()
}
