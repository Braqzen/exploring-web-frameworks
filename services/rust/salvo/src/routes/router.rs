use crate::routes::{
    handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler},
    middleware::{chaos_middleware, log_middleware},
};
use app::state::AppState;
use salvo::{Router, affix_state::inject, size_limiter::max_size as limiter};
use std::sync::{Arc, Mutex};

/// The multipler for the maximum size of a request body
const BYTES: usize = 1024;

pub fn router(state: Arc<Mutex<AppState>>) -> Router {
    // SAFETY: Nothing should have locked on boot therefore cannot panic
    let max_size = BYTES * state.lock().unwrap().config.request_size_limit as usize;

    Router::new()
        .hoop(inject(state))
        .hoop(chaos_middleware)
        .hoop(log_middleware)
        .push(
            Router::new()
                .post(post_handler)
                .hoop(limiter(max_size as u64)),
        )
        .push(
            Router::with_path("{task_id}")
                .get(get_handler)
                .delete(delete_handler)
                .push(
                    Router::new()
                        .hoop(limiter(max_size as u64))
                        .put(put_handler)
                        .patch(patch_handler),
                ),
        )
}
