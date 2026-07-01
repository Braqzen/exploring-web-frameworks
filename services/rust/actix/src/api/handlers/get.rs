use crate::{
    api::errors::{internal_server_error, task_not_found},
    state::State as ServerState,
};
use actix_web::{
    HttpResponse,
    web::{Data, ReqData},
};
use std::sync::Mutex;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(skip_all)]
pub async fn get_handler(state: Data<Mutex<ServerState>>, id: ReqData<Uuid>) -> HttpResponse {
    let id = id.into_inner();
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

            return HttpResponse::Ok().json(task);
        } else {
            drop(state);
            warn!(%id, method = "GET", "Task not found");
            return task_not_found();
        }
    }

    error!(%id, method = "GET", "Poisoned lock");

    internal_server_error()
}
