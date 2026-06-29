use crate::{state::State as ServerState, task::Task};
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use tracing::{error, info, instrument};
use uuid::Uuid;
use warp::{
    http::StatusCode,
    reply::{Reply, Response, json, reply, with_status},
};

#[instrument(skip_all)]
pub async fn post_handler(
    state: Arc<Mutex<ServerState>>,
    request: Task,
) -> Result<Response, Infallible> {
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

        return Ok(json(&id.to_string()).into_response());
    }

    error!(
        %id,
        secret = request.secret,
        operation = request.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    Ok(with_status(reply(), StatusCode::INTERNAL_SERVER_ERROR).into_response())
}
