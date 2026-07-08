use crate::routes::{errors::AppError, extractors::AppPath};
use app::state::AppState;
use poem::{
    Response,
    error::ResponseError,
    web::{Data, IntoResponse, Json},
};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};

#[poem::handler]
#[instrument(skip_all)]
pub async fn get_handler(
    Data(state): Data<&Arc<Mutex<AppState>>>,
    AppPath(id): AppPath,
) -> Response {
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

            return Json(task).into_response();
        } else {
            drop(state);
            warn!(%id, method = "GET", "Task not found");
            return AppError::TaskNotFound.as_response();
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    AppError::Internal.as_response()
}
