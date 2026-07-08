use crate::routes::{errors::AppError, extractors::AppPath};
use app::state::AppState;
use poem::{IntoResponse, Response, error::ResponseError, http::StatusCode, web::Data};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};

#[poem::handler]
#[instrument(skip_all)]
pub async fn delete_handler(
    Data(state): Data<&Arc<Mutex<AppState>>>,
    AppPath(id): AppPath,
) -> Response {
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
            return StatusCode::NO_CONTENT.into_response();
        } else {
            drop(state);
            warn!(%id, method = "DELETE", "Task not found");
            return AppError::TaskNotFound.as_response();
        }
    }

    error!(%id, method = "DELETE", "Poisoned lock");

    AppError::Internal.as_response()
}
