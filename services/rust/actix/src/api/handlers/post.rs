use crate::{api::errors::internal_server_error, state::State as ServerState, task::Task};
use actix_web::{
    HttpResponse,
    web::{Data, ReqData},
};
use serde_json::json;
use std::sync::Mutex;
use tracing::{error, info, instrument};
use uuid::Uuid;

#[instrument(skip_all)]
pub async fn post_handler(state: Data<Mutex<ServerState>>, request: ReqData<Task>) -> HttpResponse {
    let id = Uuid::new_v4();

    if let Ok(mut state) = state.lock() {
        let request = request.into_inner();
        state.tasks.insert(id, request.clone());
        drop(state);

        info!(
            %id,
            secret = request.secret.len(),
            operation = request.operation.to_string(),
            method = "POST",
            "Inserted new task"
        );

        return HttpResponse::Created().json(json!({ "id": id.to_string() }));
    }

    error!(
        %id,
        secret = request.secret.len(),
        operation = request.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    internal_server_error()
}
