use crate::{
    api::handlers::{
        delete::remove, get::fetch, patch::partial_update, post::insert, put::overwrite,
    },
    state::State,
};
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::{Arc, Mutex};

pub fn router(state: Arc<Mutex<State>>) -> Router {
    Router::new()
        .route("/", post(insert))
        .route(
            "/{task_id}",
            get(fetch)
                .put(overwrite)
                .patch(partial_update)
                .delete(remove),
        )
        .with_state(state)
}
