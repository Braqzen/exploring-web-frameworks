use crate::routes::{
    handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler},
    middleware::{chaos_middleware, log_middleware},
};
use app::state::AppState;
use salvo::{Router, affix_state::inject, size_limiter::max_size};
use std::sync::{Arc, Mutex};

// TODO: Make configurable?
/// The maximum size of a request body in bytes (64KB)
const MAX_BODY_SIZE: usize = 1024 * 64;

pub fn router(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .hoop(chaos_middleware)
        .hoop(log_middleware)
        .hoop(inject(state))
        .push(
            Router::new()
                .post(post_handler)
                .hoop(max_size(MAX_BODY_SIZE as u64)),
        )
        .push(
            Router::with_path("{task_id}")
                .get(get_handler)
                .delete(delete_handler)
                .push(
                    Router::new()
                        .hoop(max_size(MAX_BODY_SIZE as u64))
                        .put(put_handler)
                        .patch(patch_handler),
                ),
        )
}
