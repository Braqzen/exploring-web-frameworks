use crate::{
    routes::{errors::AppError, extractors::AppJson},
    state::AppState,
    task::Task,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument};
use uuid::Uuid;

#[axum::debug_handler]
#[instrument(skip_all)]
pub async fn post_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    AppJson(task): AppJson<Task>,
) -> Response {
    let id = Uuid::new_v4();

    if let Ok(mut state) = state.lock() {
        state.tasks.insert(id, task.clone());
        drop(state);

        info!(
            %id,
            secret = task.secret.len(),
            operation = task.operation.to_string(),
            method = "POST",
            "Inserted new task"
        );

        return (StatusCode::CREATED, Json(json!({"id": id.to_string()}))).into_response();
    }

    error!(
        %id,
        secret = task.secret.len(),
        operation = task.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    AppError::Internal.into_response()
}
