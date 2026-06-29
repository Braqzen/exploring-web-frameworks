use crate::api::{
    errors::{internal_server_error, task_not_found},
    handlers::{state, task_id},
};
use salvo::{Depot, Response, http::StatusCode};
use tracing::{error, info, instrument, warn};

#[salvo::handler]
#[instrument(skip_all)]
pub async fn delete_handler(depot: &mut Depot, res: &mut Response) {
    let state = state(depot, "DELETE");
    let id = task_id(depot, "DELETE");

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
            res.status_code(StatusCode::NO_CONTENT);
            return;
        } else {
            drop(state);
            warn!(%id, method = "DELETE", "Task not found");
            task_not_found(res);
            return;
        }
    }

    error!(%id, method = "DELETE", "Poisoned lock");

    internal_server_error(res);
}
