use crate::routes::{errors::AppError, extractors::AppPath, handlers::state};
use salvo::{Depot, Response, Writer, http::StatusCode};
use tracing::{error, info, instrument, warn};

#[salvo::handler]
#[instrument(skip_all)]
pub async fn delete_handler(res: &mut Response, depot: &mut Depot, id: AppPath) {
    let state = state(depot, "DELETE");
    let id = id.task_id;

    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.remove(&id) {
            drop(state);
            info!(
                %id,
                secret = task.secret.len(),
                operation = task.operation.to_string(),
                method = "DELETE",
                "Removed task"
            );
            res.status_code(StatusCode::NO_CONTENT);
            return;
        } else {
            drop(state);
            warn!(%id, method = "DELETE", "Task not found");
            AppError::TaskNotFound.render(res);
            return;
        }
    }

    error!(%id, method = "DELETE", "Poisoned lock");

    AppError::Internal.render(res);
}
