use crate::{
    api::{
        handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler},
        validate_request,
    },
    state::State,
};
use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};
use std::sync::{Arc, Mutex};

pub fn router(state: Arc<Mutex<State>>) -> Router {
    Router::new()
        .route("/", post(post_handler))
        .route(
            "/{task_id}",
            get(get_handler)
                .put(put_handler)
                .patch(patch_handler)
                .delete(delete_handler),
        )
        .with_state(state)
        .layer(from_fn(validate_request))
}
