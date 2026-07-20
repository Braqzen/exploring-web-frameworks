use crate::routes::{
    handlers::{
        delete_handler, get_handler, invalid_method_handler, invalid_path_handler, patch_handler,
        post_handler, put_handler,
    },
    middleware::{chaos_middleware, log_middleware},
};
use app::state::AppState;
use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware::{from_fn, from_fn_with_state},
    routing::{get, post},
};
use std::sync::{Arc, Mutex};

/// The multipler for the maximum size of a request body
const BYTES: usize = 1024;

pub fn router(state: Arc<Mutex<AppState>>) -> Router {
    // SAFETY: Nothing should have locked on boot therefore cannot panic
    let max_size = BYTES * state.lock().unwrap().config.request_size_limit as usize;

    Router::new()
        .route("/", post(post_handler))
        .route(
            "/{task_id}",
            get(get_handler)
                .put(put_handler)
                .patch(patch_handler)
                .delete(delete_handler),
        )
        .with_state(state.clone())
        .fallback(invalid_path_handler)
        .method_not_allowed_fallback(invalid_method_handler)
        .layer(from_fn_with_state(state, chaos_middleware))
        .layer(from_fn(log_middleware))
        .layer(DefaultBodyLimit::max(max_size))
}
