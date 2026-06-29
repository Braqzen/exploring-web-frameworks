use crate::{
    api::{
        handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler},
        middleware::validate_request,
    },
    state::State,
};
use poem::{EndpointExt, IntoEndpoint, Route, get, post};
use std::sync::{Arc, Mutex};

pub fn router(state: Arc<Mutex<State>>) -> impl IntoEndpoint {
    Route::new()
        .at("/", post(post_handler))
        .at(
            "/:task_id",
            get(get_handler)
                .put(put_handler)
                .patch(patch_handler)
                .delete(delete_handler),
        )
        .data(state)
        .around(validate_request)
}
