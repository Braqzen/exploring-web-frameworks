use crate::routes::handlers::{
    delete_handler, get_handler, invalid_method_handler, invalid_path_handler, patch_handler,
    post_handler, put_handler,
};
use actix_web::web::{self, JsonConfig, ServiceConfig, delete, get, patch, post, put, resource};
use app::state::AppState;
use std::sync::{Arc, Mutex};

/// The multipler for the maximum size of a request body
const BYTES: usize = 1024;

pub fn router(config: &mut ServiceConfig, state: Arc<Mutex<AppState>>) {
    // SAFETY: Nothing should have locked on boot therefore cannot panic
    let max_size = BYTES * state.lock().unwrap().config.request_size_limit as usize;

    config
        .service(
            resource("/")
                .route(post().to(post_handler))
                .default_service(web::to(invalid_method_handler)),
        )
        .service(
            resource("/{task_id}")
                .route(get().to(get_handler))
                .route(put().to(put_handler))
                .route(patch().to(patch_handler))
                .route(delete().to(delete_handler))
                .default_service(web::to(invalid_method_handler)),
        )
        .default_service(web::to(invalid_path_handler))
        .app_data(JsonConfig::default().limit(max_size));
}
