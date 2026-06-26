use crate::{state::State as ServerState, task::Task};
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use std::sync::Mutex;
use tracing::{error, info, instrument};
use uuid::Uuid;

#[instrument(name = "insert", skip_all)]
pub async fn insert(state: Data<Mutex<ServerState>>, request: Json<Task>) -> HttpResponse {
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

        return HttpResponse::Ok().json(id.to_string());
    }

    error!(
        %id,
        secret = request.secret,
        operation = request.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    HttpResponse::InternalServerError().finish()
}
