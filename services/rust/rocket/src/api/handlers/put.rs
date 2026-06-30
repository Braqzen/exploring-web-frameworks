use crate::{
    api::{
        errors::{internal_server_error, task_not_found},
        guard::Extract,
    },
    state::State as ServerState,
    task::Task,
};
use rocket::{State, http::Status, put, serde::json::Json};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[put("/<id>", data = "<request>")]
#[instrument(skip_all)]
pub async fn put_handler(
    id: Uuid,
    state: &State<Arc<Mutex<ServerState>>>,
    request: Extract<Task>,
) -> (Status, Json<Value>) {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let previous_task = task.clone();
            *task = request.into_inner();

            info!(
                %id,
                from_secret = previous_task.secret,
                to_secret = task.secret,
                from_operation = previous_task.operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PUT",
                "Overwrote task"
            );

            return (Status::Ok, Json(json!(task.to_owned())));
        } else {
            drop(state);
            warn!(%id, method = "PUT", "Task not found");
            return task_not_found();
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    internal_server_error()
}
