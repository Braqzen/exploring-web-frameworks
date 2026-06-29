use crate::{api::handlers::state, task::Task};
use salvo::{Depot, Request, http::StatusCode, writing::Json};
use serde_json::Value;
use tracing::{error, info, instrument};
use uuid::Uuid;

#[salvo::handler]
#[instrument(skip_all)]
pub async fn post_handler(
    depot: &mut Depot,
    request: &mut Request,
) -> Result<Json<Value>, StatusCode> {
    let state = state(depot);

    let request = request
        .parse_json::<Task>()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

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

        return Ok(Json(Value::String(id.to_string())));
    }

    error!(
        %id,
        secret = request.secret,
        operation = request.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
