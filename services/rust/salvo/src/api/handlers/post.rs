use crate::api::{
    errors::internal_server_error,
    handlers::{state, task},
};
use salvo::{Depot, Response, http::StatusCode, writing::Json};
use serde_json::json;
use tracing::{error, info, instrument};
use uuid::Uuid;

#[salvo::handler]
#[instrument(skip_all)]
pub async fn post_handler(depot: &mut Depot, res: &mut Response) {
    let state = state(depot, "POST");
    let request = task(depot, "POST");

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

        res.stuff(StatusCode::CREATED, Json(json!({"id": id.to_string()})));
        return;
    }

    error!(
        %id,
        secret = request.secret,
        operation = request.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    internal_server_error(res)
}
