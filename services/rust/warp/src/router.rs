use crate::{
    api::handlers::{delete_handler, get_handler, patch_handler, post_handler, put_handler},
    state::State,
    task::Task,
};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use warp::{Filter, reject::Rejection, reply::Reply};

pub fn router(
    state: Arc<Mutex<State>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let post = warp::path::end()
        .and(warp::post())
        .and(warp::any().map({
            let state = state.clone();
            move || state.clone()
        }))
        .and(warp::body::json::<Task>())
        .and_then(post_handler);

    let put = warp::path!(Uuid)
        .and(warp::put())
        .and(warp::any().map({
            let state = state.clone();
            move || state.clone()
        }))
        .and(warp::body::json::<Task>())
        .and_then(put_handler);

    let delete = warp::path!(Uuid)
        .and(warp::delete())
        .and(warp::any().map({
            let state = state.clone();
            move || state.clone()
        }))
        .and_then(delete_handler);

    let get = warp::path!(Uuid)
        .and(warp::get())
        .and(warp::any().map({
            let state = state.clone();
            move || state.clone()
        }))
        .and_then(get_handler);

    let patch = warp::path!(Uuid)
        .and(warp::patch())
        .and(warp::any().map({
            let state = state.clone();
            move || state.clone()
        }))
        .and(warp::body::json::<Value>())
        .and_then(patch_handler);

    post.or(put).or(delete).or(get).or(patch)
}
