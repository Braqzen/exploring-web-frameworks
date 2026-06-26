use crate::state::State as ServerState;
use actix_web::{
    HttpResponse,
    web::{Data, Path},
};
use std::sync::Mutex;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(name = "delete", skip_all)]
pub async fn remove(state: Data<Mutex<ServerState>>, id: Path<(Uuid,)>) -> HttpResponse {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.remove(&id.0) {
            drop(state);
            info!(
                id = %id.0,
                secret = task.secret,
                operation = task.operation.to_string(),
                method = "DELETE",
                "Removed task"
            );
            return HttpResponse::NoContent().finish();
        } else {
            drop(state);
            warn!(id = %id.0, method = "DELETE", "Task not found");
            return HttpResponse::NotFound().finish();
        }
    }

    error!(id = %id.0, method = "DELETE", "Poisoned lock");

    HttpResponse::InternalServerError().finish()
}
