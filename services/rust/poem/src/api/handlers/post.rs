use crate::{api::errors::internal_server_error, state::State as ServerState, task::Task};
use poem::{
    Response,
    http::StatusCode,
    web::{Data, IntoResponse, Json},
};
use serde_json::json;
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument};
use uuid::Uuid;

#[poem::handler]
#[instrument(skip_all)]
pub async fn post_handler(
    Data(state): Data<&Arc<Mutex<ServerState>>>,
    Data(task): Data<&Task>,
) -> Response {
    let id = Uuid::new_v4();

    if let Ok(mut state) = state.lock() {
        state.tasks.insert(id, task.clone());
        drop(state);

        info!(
            %id,
            secret = task.secret,
            operation = task.operation.to_string(),
            method = "POST",
            "Inserted new task"
        );

        return (StatusCode::CREATED, Json(json!({"id": id.to_string()}))).into_response();
    }

    error!(
        %id,
        secret = task.secret,
        operation = task.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    internal_server_error()
}
