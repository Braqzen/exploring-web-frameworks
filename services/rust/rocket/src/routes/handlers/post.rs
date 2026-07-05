use crate::{
    routes::{errors::AppError, extractors::AppJson, guards::ChaosGuard},
    state::AppState,
    task::Task,
};
use rocket::{State, http::Status, post, serde::json::Json};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument};
use uuid::Uuid;

#[post("/", data = "<body>")]
#[instrument(skip_all)]
pub async fn post_handler(
    _guard: ChaosGuard,
    state: &State<Arc<Mutex<AppState>>>,
    body: AppJson<Task>,
) -> (Status, Json<Value>) {
    let id = Uuid::new_v4();
    let request = body.into_inner();

    if let Ok(mut state) = state.lock() {
        state.tasks.insert(id, request.clone());
        drop(state);

        info!(
            %id,
            secret = request.secret.len(),
            operation = request.operation.to_string(),
            method = "POST",
            "Inserted new task"
        );

        return (Status::Created, Json(json!({"id": id.to_string()})));
    }

    error!(
        %id,
        secret = request.secret.len(),
        operation = request.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    AppError::Internal.into_response()
}
