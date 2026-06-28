use crate::{
    api::handlers::{state, task_id},
    task::Task,
};
use salvo::{Depot, Request, http::StatusCode, writing::Json};
use tracing::{error, info, instrument, warn};

#[instrument(name = "get", skip_all)]
#[salvo::handler]
pub async fn fetch(depot: &mut Depot, request: &mut Request) -> Result<Json<Task>, StatusCode> {
    let state = state(depot);
    let id = task_id(request);
    if id.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let id = id.unwrap();

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
