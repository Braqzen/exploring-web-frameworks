use crate::{
    api::errors::{internal_server_error, task_not_found},
    state::State as ServerState,
    task::PatchedTask,
};
use actix_web::{
    HttpResponse,
    web::{Data, ReqData},
};
use std::sync::Mutex;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(skip_all)]
pub async fn patch_handler(
    state: Data<Mutex<ServerState>>,
    id: ReqData<Uuid>,
    request: ReqData<PatchedTask>,
) -> HttpResponse {
    let id = id.into_inner();

    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let request = request.into_inner();

            // Code assumes only operation is changed
            let previous_operation = task.operation.clone();
            task.operation = request.operation;

            info!(
                %id,
                secret = task.secret,
                from_operation = previous_operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PATCH",
                "Patched task"
            );

            return HttpResponse::Ok().json(task);
        } else {
            drop(state);
            warn!(%id, method = "PATCH", "Task not found");
            return task_not_found();
        }
    }

    error!(%id, method = "PATCH", "Poisoned lock");

    internal_server_error()
}
