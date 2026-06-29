use crate::{
    api::handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler},
    state::State,
};
use salvo::{Router, affix_state::inject};
use std::sync::{Arc, Mutex};

pub fn router(state: Arc<Mutex<State>>) -> Router {
    Router::new()
        .post(post_handler)
        .push(
            Router::with_path("{task_id}")
                .get(get_handler)
                .put(put_handler)
                .patch(patch_handler)
                .delete(delete_handler),
        )
        .hoop(inject(state))
}
