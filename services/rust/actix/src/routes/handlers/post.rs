use crate::routes::{errors::AppError, extractors::AppJson};
use actix_web::{HttpResponse, ResponseError, web::Data};
use app::{state::AppState, task::Task};
use serde_json::json;
use std::sync::Mutex;
use tracing::{error, info, instrument};
use uuid::Uuid;

#[instrument(skip_all)]
pub async fn post_handler(
    state: Data<Mutex<AppState>>,
    AppJson(new_task): AppJson<Task>,
) -> HttpResponse {
    let id = Uuid::new_v4();

    if let Ok(mut state) = state.lock() {
        state.tasks.insert(id, new_task.clone());
        drop(state);

        info!(
            %id,
            secret = new_task.secret.len(),
            operation = new_task.operation.to_string(),
            method = "POST",
            "Inserted new task"
        );

        return HttpResponse::Created().json(json!({ "id": id.to_string() }));
    }

    error!(
        %id,
        secret = new_task.secret.len(),
        operation = new_task.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    AppError::Internal.error_response()
}
