use crate::api::handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler};
use actix_web::web::{ServiceConfig, delete, get, patch, post, put, resource};

pub fn router(config: &mut ServiceConfig) {
    config.route("/", post().to(post_handler)).service(
        resource("/{task_id}")
            .route(get().to(get_handler))
            .route(put().to(put_handler))
            .route(patch().to(patch_handler))
            .route(delete().to(delete_handler)),
    );
}
