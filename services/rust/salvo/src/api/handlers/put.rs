use crate::api::{
    errors::{internal_server_error, task_not_found},
    handlers::{state, task, task_id},
};
use salvo::{Depot, Response, http::StatusCode, writing::Json};
use tracing::{error, info, instrument, warn};

#[salvo::handler]
#[instrument(skip_all)]
pub async fn put_handler(depot: &mut Depot, res: &mut Response) {
    let state = state(depot, "PUT");
    let id = task_id(depot, "PUT");
    let request = task(depot, "PUT");

    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let previous_task = task.clone();
            *task = request.to_owned();

            info!(
                %id,
                from_secret = previous_task.secret.len(),
                to_secret = task.secret.len(),
                from_operation = previous_task.operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PUT",
                "Overwrote task"
            );

            res.stuff(StatusCode::OK, Json(task.to_owned()));
            return;
        } else {
            drop(state);
            warn!(%id, method = "PUT", "Task not found");
            task_not_found(res);
            return;
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    internal_server_error(res)
}
