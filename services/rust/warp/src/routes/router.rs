use crate::routes::{
    errors::AppError,
    filters::{chaos_filter, handle_rejection, log_filter, patched_body, task_body, task_id},
    handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler},
};
use app::state::AppState;
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use warp::{Filter, reply::Reply};

/// The multipler for the maximum size of a request body
const BYTES: usize = 1024;

pub fn router(
    state: Arc<Mutex<AppState>>,
) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    let max_size = BYTES * state.lock().unwrap().config.request_size_limit as usize;

    let warp_state = warp::any().map({
        let state = state.clone();
        move || state.clone()
    });

    let invalid_root = warp::path::end()
        .and(
            warp::get()
                .or(warp::put())
                .or(warp::patch())
                .or(warp::delete())
                .or(warp::head()),
        )
        .and_then(|_| async { Ok::<_, Infallible>(AppError::InvalidMethod.into_response()) });

    let post = warp::post()
        .and(warp::path::end())
        .and(warp_state.clone())
        .and(task_body(max_size))
        .and_then(post_handler);

    let put = warp::put()
        .and(task_id())
        .and(warp_state.clone())
        .and(task_body(max_size))
        .and_then(put_handler);

    let delete = warp::delete()
        .and(task_id())
        .and(warp_state.clone())
        .and_then(delete_handler);

    let get = warp::get()
        .and(task_id())
        .and(warp_state.clone())
        .and_then(get_handler);

    let patch = warp::patch()
        .and(task_id())
        .and(warp_state)
        .and(patched_body(max_size))
        .and_then(patch_handler);

    log_filter()
        .and(chaos_filter(state.clone()))
        .and(invalid_root.or(post).or(put).or(delete).or(get).or(patch))
        .recover(handle_rejection)
}
