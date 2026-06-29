use crate::{state::State as ServerState, task::Task};
use rocket::{State, http::Status, post, serde::json::Json};
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument};
use uuid::Uuid;

#[post("/", data = "<request>")]
#[instrument(skip_all)]
pub async fn post_handler(
    state: &State<Arc<Mutex<ServerState>>>,
    request: Json<Task>,
) -> Result<Json<String>, Status> {
    let id = Uuid::new_v4();

    if let Ok(mut state) = state.lock() {
        let request = request.into_inner();
        state.tasks.insert(id, request.clone());
        drop(state);

        info!(
            %id,
            secret = request.secret,
            operation = request.operation.to_string(),
            method = "POST",
            "Inserted new task"
        );

        return Ok(Json(id.to_string()));
    }

    error!(
        %id,
        secret = request.secret,
        operation = request.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    return Err(Status::InternalServerError);
}
