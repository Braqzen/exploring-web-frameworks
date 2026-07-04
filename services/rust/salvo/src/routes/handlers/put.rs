use crate::{
    routes::{
        errors::AppError,
        extractors::{AppJson, AppPath},
        handlers::state,
    },
    task::Task,
};
use salvo::{Depot, Response, Writer, http::StatusCode, writing::Json};
use tracing::{error, info, instrument, warn};

#[salvo::handler]
#[instrument(skip_all)]
pub async fn put_handler(
    depot: &mut Depot,
    res: &mut Response,
    id: AppPath,
    new_task: AppJson<Task>,
) {
    let state = state(depot, "PUT");
    let id = id.task_id;
    let new_task = new_task.value;

    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let previous_task = task.clone();
            *task = new_task.to_owned();

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
            AppError::TaskNotFound.render(res);
            return;
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    AppError::Internal.render(res);
}
