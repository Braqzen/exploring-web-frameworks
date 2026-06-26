use crate::state::State as ServerState;
use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use serde_json::Value;
use std::sync::Mutex;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

#[instrument(name = "patch", skip_all)]
pub async fn partial_update(
    state: Data<Mutex<ServerState>>,
    id: Path<(Uuid,)>,
    request: Json<Value>,
) -> HttpResponse {
    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id.0) {
            // Code assumes only operation is changed
            if let Some(operation) = request.get("operation").and_then(|v| v.as_str()) {
                if let Ok(operation) = operation.try_into() {
                    let previous_operation = task.operation.clone();
                    task.operation = operation;

                    info!(
                        id = %id.0,
                        secret = task.secret,
                        from_operation = previous_operation.to_string(),
                        to_operation = task.operation.to_string(),
                        method = "PATCH",
                        "Patched task"
                    );

                    return HttpResponse::Ok().json(task);
                }
            }
            return HttpResponse::BadRequest().finish();
        } else {
            drop(state);
            warn!(id = %id.0, method = "PATCH", "Task not found");
            return HttpResponse::NotFound().finish();
        }
    }

    error!(id = %id.0, method = "PATCH", "Poisoned lock");

    HttpResponse::InternalServerError().finish()
}
