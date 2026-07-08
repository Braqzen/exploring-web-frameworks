use crate::routes::{errors::AppError, extractors::AppPath};
use actix_web::{HttpResponse, ResponseError, web::Data};
use app::state::AppState;
use std::sync::Mutex;
use tracing::{error, info, instrument, warn};

#[instrument(skip_all)]
pub async fn get_handler(state: Data<Mutex<AppState>>, AppPath(id): AppPath) -> HttpResponse {
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

            return HttpResponse::Ok().json(task);
        } else {
            drop(state);
            warn!(%id, method = "GET", "Task not found");
            return AppError::TaskNotFound.error_response();
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    AppError::Internal.error_response()
}
