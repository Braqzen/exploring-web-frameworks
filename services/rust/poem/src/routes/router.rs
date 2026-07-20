use crate::routes::{
    handlers::{
        delete_handler, get_handler, invalid_method_handler, invalid_path_handler, patch_handler,
        post_handler, put_handler,
    },
    middleware::{chaos_middleware, log_middleware},
};
use app::state::AppState;
use poem::{EndpointExt, IntoEndpoint, Middleware, Route, get, middleware::SizeLimit, post};
use std::sync::{Arc, Mutex};

/// The multipler for the maximum size of a request body
const BYTES: usize = 1024;

pub fn router(state: Arc<Mutex<AppState>>) -> impl IntoEndpoint {
    // SAFETY: Nothing should have locked on boot therefore cannot panic
    let max_size = BYTES * state.lock().unwrap().config.request_size_limit as usize;

    Route::new()
        .at("/", post(post_handler).with(SizeLimit::new(max_size)))
        .at(
            "/:task_id",
            get(get_handler)
                .put(SizeLimit::new(max_size).transform(put_handler.into_endpoint()))
                .patch(SizeLimit::new(max_size).transform(patch_handler.into_endpoint()))
                .delete(delete_handler),
        )
        .catch_error(invalid_path_handler)
        .catch_error(invalid_method_handler)
        .around(chaos_middleware)
        .around(log_middleware)
        .data(state)
}
