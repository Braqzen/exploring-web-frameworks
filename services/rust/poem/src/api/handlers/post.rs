use crate::{state::State as ServerState, task::Task};
use poem::{
    http::StatusCode,
    web::{Data, Json},
};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument};
use uuid::Uuid;

#[instrument(name = "insert", skip_all)]
#[poem::handler]
pub async fn insert(
    Data(state): Data<&Arc<Mutex<ServerState>>>,
    Json(request): Json<Task>,
) -> Result<Json<Value>, StatusCode> {
    let id = Uuid::new_v4();

    if let Ok(mut state) = state.lock() {
        state.tasks.insert(id, request.clone());
        drop(state);

        info!(
            %id,
            secret = request.secret,
            operation = request.operation.to_string(),
            method = "POST",
            "Inserted new task"
        );

        return Ok(Json(Value::String(id.to_string())));
    }

    error!(
        %id,
        secret = request.secret,
        operation = request.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
