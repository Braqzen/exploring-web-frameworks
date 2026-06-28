use crate::{
    api::handlers::{state, task_id},
    task::Task,
};
use salvo::{Depot, Request, http::StatusCode, writing::Json};
use tracing::{error, info, instrument, warn};

#[instrument(name = "put", skip_all)]
#[salvo::handler]
pub async fn overwrite(depot: &mut Depot, request: &mut Request) -> Result<Json<Task>, StatusCode> {
    let state = state(depot);
    let id = task_id(request);
    if id.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let id = id.unwrap();

    let request = request
        .parse_json::<Task>()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    if let Ok(mut state) = state.lock() {
        if let Some(task) = state.tasks.get_mut(&id) {
            let previous_task = task.clone();
            *task = request.clone();

            info!(
                %id,
                from_secret = previous_task.secret,
                to_secret = task.secret,
                from_operation = previous_task.operation.to_string(),
                to_operation = task.operation.to_string(),
                method = "PUT",
                "Overwrote task"
            );

            return Ok(Json(task.to_owned()));
        } else {
            drop(state);
            warn!(%id, method = "PUT", "Task not found");
            return Err(StatusCode::NOT_FOUND);
        }
    }

    error!(%id, method = "PUT", "Poisoned lock");

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
