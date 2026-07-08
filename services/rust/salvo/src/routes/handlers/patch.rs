use crate::routes::{
    errors::AppError,
    extractors::{AppJson, AppPath},
    handlers::state,
};
use app::task::PatchedTask;
use salvo::{Depot, Response, Writer, http::StatusCode, writing::Json};
use tracing::{error, info, instrument, warn};

#[salvo::handler]
#[instrument(skip_all)]
pub async fn patch_handler(
    depot: &mut Depot,
    res: &mut Response,
    id: AppPath,
    new_task: AppJson<PatchedTask>,
) {
    let state = state(depot, "PATCH");
    let id = id.task_id;
    let new_task = new_task.value;

    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let previous_operation = task.operation.clone();
            task.operation = new_task.operation.clone();

            info!(
                %id,
                secret = task.secret.len(),
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
            AppError::TaskNotFound.render(res);
            return;
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    AppError::Internal.render(res);
}
