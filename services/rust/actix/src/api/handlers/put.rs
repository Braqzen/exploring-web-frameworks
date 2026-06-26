use crate::{state::State as ServerState, task::Task};
use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use std::sync::Mutex;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(name = "put", skip_all)]
pub async fn overwrite(
    state: Data<Mutex<ServerState>>,
    id: Path<(Uuid,)>,
    request: Json<Task>,
) -> HttpResponse {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id.0) {
            let previous_task = task.clone();
            *task = request.clone();

            info!(
                id = %id.0,
                from_secret = previous_task.secret,
                to_secret = task.secret,
                from_operation = previous_task.operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PUT",
                "Overwrote task"
            );

            return HttpResponse::Ok().json(task);
        } else {
            drop(state);
            warn!(id = %id.0, method = "PUT", "Task not found");
            return HttpResponse::NotFound().finish();
        }
    }

    error!(id = %id.0, method = "PUT", "Poisoned lock");

    HttpResponse::InternalServerError().finish()
}
