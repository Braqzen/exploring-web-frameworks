use crate::routes::{
    errors::AppError,
    extractors::{AppJson, AppPath},
};
use actix_web::{HttpResponse, ResponseError, web::Data};
use app::{state::AppState, task::Task};
use std::sync::Mutex;
use tracing::{error, info, instrument, warn};

#[instrument(skip_all)]
pub async fn put_handler(
    state: Data<Mutex<AppState>>,
    AppPath(id): AppPath,
    AppJson(new_task): AppJson<Task>,
) -> HttpResponse {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let request = new_task;

            let previous_task = task.clone();
            *task = request.clone();

            info!(
                %id,
                from_secret = previous_task.secret.len(),
                to_secret = task.secret.len(),
                from_operation = previous_task.operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PUT",
                "Overwrote task"
            );

            return HttpResponse::Ok().json(task);
        } else {
            drop(state);
            warn!(%id, method = "PUT", "Task not found");
            return AppError::TaskNotFound.error_response();
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    AppError::Internal.error_response()
}
