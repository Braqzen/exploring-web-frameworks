use crate::{
    routes::{errors::AppError, extractors::AppJson, handlers::state},
    task::Task,
};
use salvo::{Depot, Response, Writer, http::StatusCode, writing::Json};
use serde_json::json;
use tracing::{error, info, instrument};
use uuid::Uuid;

#[salvo::handler]
#[instrument(skip_all)]
pub async fn post_handler(depot: &mut Depot, res: &mut Response, new_task: AppJson<Task>) {
    let state = state(depot, "POST");
    let new_task = new_task.value;

    let id = Uuid::new_v4();

    if let Ok(mut state) = state.lock() {
        state.tasks.insert(id, new_task.clone());
        drop(state);

        info!(
            %id,
            secret = new_task.secret.len(),
            operation = new_task.operation.to_string(),
            method = "POST",
            "Inserted new task"
        );

        res.stuff(StatusCode::CREATED, Json(json!({"id": id.to_string()})));
        return;
    }

    error!(
        %id,
        secret = new_task.secret.len(),
        operation = new_task.operation.to_string(),
        method = "POST",
        "Poisoned lock"
    );

    AppError::Internal.render(res);
}
