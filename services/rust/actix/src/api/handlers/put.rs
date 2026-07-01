use crate::{
    api::errors::{internal_server_error, task_not_found},
    state::State as ServerState,
    task::Task,
};
use actix_web::{
    HttpResponse,
    web::{Data, ReqData},
};
use std::sync::Mutex;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(skip_all)]
pub async fn put_handler(
    state: Data<Mutex<ServerState>>,
    id: ReqData<Uuid>,
    request: ReqData<Task>,
) -> HttpResponse {
    let id = id.into_inner();

    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let request = request.into_inner();

            let previous_task = task.clone();
            *task = request.clone();

            info!(
                %id,
                from_secret = previous_task.secret.len(),
                to_secret = task.secret.len(),
                from_operation = previous_task.operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PUT",
                "Overwrote task"
            );

            return HttpResponse::Ok().json(task);
        } else {
            drop(state);
            warn!(%id, method = "PUT", "Task not found");
            return task_not_found();
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    internal_server_error()
}
