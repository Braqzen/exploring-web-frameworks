use crate::routes::{errors::AppError, extractors::AppPath, handlers::state};
use salvo::{Depot, Response, Writer, http::StatusCode, writing::Json};
use tracing::{error, info, instrument, warn};

#[salvo::handler]
#[instrument(skip_all)]
pub async fn get_handler(depot: &mut Depot, res: &mut Response, id: AppPath) {
    let state = state(depot, "GET");
    let id = id.task_id;

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
            AppError::TaskNotFound.render(res);
            return;
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    AppError::Internal.render(res);
}
