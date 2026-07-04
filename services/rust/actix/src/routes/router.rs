use crate::routes::handlers::{
    delete_handler, get_handler, invalid_method_handler, invalid_path_handler, patch_handler,
    post_handler, put_handler,
};
use actix_web::web::{self, JsonConfig, ServiceConfig, delete, get, patch, post, put, resource};

// TODO: Make configurable?
/// The maximum size of a request body in bytes (64KB)
const MAX_BODY_SIZE: usize = 1024 * 64;

pub fn router(config: &mut ServiceConfig) {
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
        .app_data(JsonConfig::default().limit(MAX_BODY_SIZE));
}
