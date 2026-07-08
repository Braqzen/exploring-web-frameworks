use crate::routes::{
    errors::AppError,
    extractors::{AppJson, AppPath},
};
use actix_web::{HttpResponse, ResponseError, web::Data};
use app::{state::AppState, task::PatchedTask};
use std::sync::Mutex;
use tracing::{error, info, instrument, warn};

#[instrument(skip_all)]
pub async fn patch_handler(
    state: Data<Mutex<AppState>>,
    AppPath(id): AppPath,
    AppJson(new_task): AppJson<PatchedTask>,
) -> HttpResponse {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let request = new_task;

            let previous_operation = task.operation.clone();
            task.operation = request.operation;

            info!(
                %id,
                secret = task.secret.len(),
                from_operation = previous_operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PATCH",
                "Patched task"
            );

            return HttpResponse::Ok().json(task);
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            return AppError::TaskNotFound.error_response();
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    AppError::Internal.error_response()
}
