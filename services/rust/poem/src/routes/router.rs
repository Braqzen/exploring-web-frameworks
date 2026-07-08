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

// TODO: Make configurable?
/// The maximum size of a request body in bytes (64KB)
const MAX_BODY_SIZE: usize = 1024 * 64;

pub fn router(state: Arc<Mutex<AppState>>) -> impl IntoEndpoint {
    Route::new()
        .at("/", post(post_handler).with(SizeLimit::new(MAX_BODY_SIZE)))
        .at(
            "/:task_id",
            get(get_handler)
                .put(SizeLimit::new(MAX_BODY_SIZE).transform(put_handler.into_endpoint()))
                .patch(SizeLimit::new(MAX_BODY_SIZE).transform(patch_handler.into_endpoint()))
                .delete(delete_handler),
        )
        .data(state)
        .catch_error(invalid_path_handler)
        .catch_error(invalid_method_handler)
        .around(chaos_middleware)
        .around(log_middleware)
}
