use crate::api::handlers::{state, task_id};
use salvo::{Depot, Request, http::StatusCode};
use tracing::{error, info, instrument, warn};

#[salvo::handler]
#[instrument(skip_all)]
pub async fn delete_handler(
    depot: &mut Depot,
    request: &mut Request,
) -> Result<StatusCode, StatusCode> {
    let state = state(depot);
    let id = task_id(&request);
    if id.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let id = id.unwrap();

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
