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
pub async fn delete_handler(state: Data<Mutex<ServerState>>, id: ReqData<Uuid>) -> HttpResponse {
    let id = id.into_inner();
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.remove(&id) {
            drop(state);
            info!(
                %id,
                secret = task.secret,
                operation = task.operation.to_string(),
                method = "DELETE",
                "Removed task"
            );
            return HttpResponse::NoContent().finish();
        } else {
            drop(state);
            warn!(%id, method = "DELETE", "Task not found");
            return task_not_found();
        }
    }

    error!(%id, method = "DELETE", "Poisoned lock");

    internal_server_error()
}
