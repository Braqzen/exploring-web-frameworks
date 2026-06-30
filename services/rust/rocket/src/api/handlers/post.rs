use crate::{
    api::{errors::internal_server_error, guard::Extract},
    state::State as ServerState,
    task::Task,
};
use rocket::{State, http::Status, post, serde::json::Json};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument};
use uuid::Uuid;

#[post("/", data = "<request>")]
#[instrument(skip_all)]
pub async fn post_handler(
    state: &State<Arc<Mutex<ServerState>>>,
    request: Extract<Task>,
) -> (Status, Json<Value>) {
    let id = Uuid::new_v4();
    let request = request.into_inner();

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

        return (Status::Created, Json(json!({"id": id.to_string()})));
    }

    error!(
        %id,
        secret = request.secret,
        operation = request.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    internal_server_error()
}
