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
    middleware::from_fn,
    routing::{get, post},
};
use std::sync::{Arc, Mutex};

// TODO: Make configurable?
/// The maximum size of a request body in bytes (64KB)
const MAX_BODY_SIZE: usize = 1024 * 64;

pub fn router(state: Arc<Mutex<AppState>>) -> Router {
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
        .fallback(invalid_path_handler)
        .method_not_allowed_fallback(invalid_method_handler)
        .layer(from_fn(chaos_middleware))
        .layer(from_fn(log_middleware))
        .layer(DefaultBodyLimit::max(MAX_BODY_SIZE))
}
