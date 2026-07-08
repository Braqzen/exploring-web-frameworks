use crate::routes::{errors::AppError, extractors::AppPath};
use actix_web::{HttpResponse, ResponseError, web::Data};
use app::state::AppState;
use std::sync::Mutex;
use tracing::{error, info, instrument, warn};

#[instrument(skip_all)]
pub async fn delete_handler(state: Data<Mutex<AppState>>, AppPath(id): AppPath) -> HttpResponse {
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
            return HttpResponse::NoContent().finish();
        } else {
            drop(state);
            warn!(%id, method = "DELETE", "Task not found");
            return AppError::TaskNotFound.error_response();
        }
    }

    error!(%id, method = "DELETE", "Poisoned lock");

    AppError::Internal.error_response()
}
