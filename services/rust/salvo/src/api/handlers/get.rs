use crate::api::{
    errors::{internal_server_error, task_not_found},
    handlers::{state, task_id},
};
use salvo::{Depot, Response, http::StatusCode, writing::Json};
use tracing::{error, info, instrument, warn};

#[salvo::handler]
#[instrument(skip_all)]
pub async fn get_handler(depot: &mut Depot, res: &mut Response) {
    let state = state(depot, "GET");
    let id = task_id(depot, "GET");

    if let Ok(state) = state.lock() {
        if let Some(task) = state.tasks.get(&id).cloned() {
            drop(state);
            info!(
                %id,
                secret = task.secret.len(),
                operation = task.operation.to_string(),
                method = "GET",
                "Retrieved task"
            );

            res.stuff(StatusCode::OK, Json(task));
            return;
        } else {
            drop(state);
            warn!(%id, method = "GET", "Task not found");
            task_not_found(res);
            return;
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    internal_server_error(res)
}
