use crate::{
    api::handlers::{state, task_id},
    task::Task,
};
use salvo::{Depot, Request, http::StatusCode, writing::Json};
use serde_json::Value;
use tracing::{error, info, instrument, warn};

#[salvo::handler]
#[instrument(skip_all)]
pub async fn patch_handler(
    depot: &mut Depot,
    request: &mut Request,
) -> Result<Json<Task>, StatusCode> {
    let state = state(depot);
    let id = task_id(request);
    if id.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let id = id.unwrap();

    let request = request
        .parse_json::<Value>()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

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
