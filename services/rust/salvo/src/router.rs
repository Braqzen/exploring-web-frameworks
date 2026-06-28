use crate::{
    api::handlers::{
        delete::remove, get::fetch, patch::partial_update, post::insert, put::overwrite,
    },
    state::State,
};
use salvo::{Router, affix_state::inject};
use std::sync::{Arc, Mutex};

pub fn router(state: Arc<Mutex<State>>) -> Router {
    Router::new()
        .post(insert)
        .push(
            Router::with_path("{task_id}")
                .get(fetch)
                .put(overwrite)
                .patch(partial_update)
                .delete(remove),
        )
        .hoop(inject(state))
}
