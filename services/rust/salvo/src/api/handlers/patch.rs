use crate::api::{
    errors::{internal_server_error, task_not_found},
    handlers::{patched_task, state, task_id},
};
use salvo::{Depot, Response, http::StatusCode, writing::Json};
use tracing::{error, info, instrument, warn};

#[salvo::handler]
#[instrument(skip_all)]
pub async fn patch_handler(depot: &mut Depot, res: &mut Response) {
    let state = state(depot, "PATCH");
    let id = task_id(depot, "PATCH");
    let request = patched_task(depot, "PATCH");

    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            // Code assumes only operation is changed
            let previous_operation = task.operation.clone();
            task.operation = request.operation.clone();

            info!(
                %id,
                secret = task.secret,
                from_operation = previous_operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PATCH",
                "Patched task"
            );

            res.stuff(StatusCode::OK, Json(task.to_owned()));
            return;
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            task_not_found(res);
            return;
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    internal_server_error(res);
}
