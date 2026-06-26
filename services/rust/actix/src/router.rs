use crate::api::handlers::{
    delete::remove, get::fetch, patch::partial_update, post::insert, put::overwrite,
};
use actix_web::web::{ServiceConfig, delete, get, patch, post, put, resource};

pub fn router(config: &mut ServiceConfig) {
    config.route("/", post().to(insert)).service(
        resource("/{task_id}")
            .route(get().to(fetch))
            .route(put().to(overwrite))
            .route(patch().to(partial_update))
            .route(delete().to(remove)),
    );
}
