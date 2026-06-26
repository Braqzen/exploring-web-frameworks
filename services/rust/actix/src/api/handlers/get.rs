use crate::state::State as ServerState;
use actix_web::{
    HttpResponse,
    web::{Data, Path},
};
use std::sync::Mutex;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(name = "get", skip_all)]
pub async fn fetch(state: Data<Mutex<ServerState>>, id: Path<(Uuid,)>) -> HttpResponse {
    if let Ok(state) = state.lock() {
        if let Some(task) = state.tasks.get(&id.0).cloned() {
            drop(state);
            info!(
                id = %id.0,
                secret = task.secret,
                operation = task.operation.to_string(),
                method = "GET",
                "Retrieved task"
            );

            return HttpResponse::Ok().json(task);
        } else {
            drop(state);
            warn!(id = %id.0, method = "GET", "Task not found");
            return HttpResponse::NotFound().finish();
        }
    }

    error!(id = %id.0, method = "GET", "Poisoned lock");

    HttpResponse::InternalServerError().finish()
}
