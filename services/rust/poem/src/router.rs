use crate::{
    api::handlers::{
        delete::remove, get::fetch, patch::partial_update, post::insert, put::overwrite,
    },
    state::State,
};
use poem::{EndpointExt, IntoEndpoint, Route, get, post};
use std::sync::{Arc, Mutex};

pub fn router(state: Arc<Mutex<State>>) -> impl IntoEndpoint {
    Route::new()
        .at("/", post(insert))
        .at(
            "/:task_id",
            get(fetch)
                .put(overwrite)
                .patch(partial_update)
                .delete(remove),
        )
        .data(state)
}
