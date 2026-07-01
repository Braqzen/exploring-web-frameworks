use crate::{
    api::{
        filters::{chaos, handle_rejection, patched_body, task_body, task_id},
        handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler},
    },
    state::State,
};
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use warp::{Filter, reply::Reply};

pub fn router(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    let warp_state = warp::any().map({
        let state = state.clone();
        move || state.clone()
    });

    let chaos = chaos();

    let post = warp::post()
        .and(warp::path::end())
        .and(warp_state.clone())
        .and(chaos.clone())
        .and(task_body())
        .and_then(post_handler);

    let put = warp::put()
        .and(task_id())
        .and(warp_state.clone())
        .and(chaos.clone())
        .and(task_body())
        .and_then(put_handler);

    let delete = warp::delete()
        .and(task_id())
        .and(warp_state.clone())
        .and(chaos.clone())
        .and_then(delete_handler);

    let get = warp::get()
        .and(task_id())
        .and(warp_state.clone())
        .and(chaos.clone())
        .and_then(get_handler);

    let patch = warp::patch()
        .and(task_id())
        .and(warp_state)
        .and(patched_body())
        .and(chaos.clone())
        .and_then(patch_handler);

    post.or(put)
        .or(delete)
        .or(get)
        .or(patch)
        .recover(handle_rejection)
}
