use crate::routes::errors::AppError;
use app::task::{PatchedTask, Task};
use serde::de::DeserializeOwned;
use tracing::{instrument, warn};
use uuid::Uuid;
use warp::{Filter, filters::path::FullPath, http::Method, hyper::body::Bytes, reject::Rejection};

/// Wrapper creating UUID or returning an error response
pub fn task_id() -> impl Filter<Extract = (Uuid,), Error = Rejection> + Copy {
    warp::path::full()
        .and(warp::method())
        .and_then(parse_task_id)
}

pub fn task_body(max_size: usize) -> impl Filter<Extract = (Task,), Error = Rejection> + Copy {
    warp::path::full()
        .and(warp::method())
        .and(warp::body::bytes())
        .and(warp::any().map(move || max_size))
        .and_then(parse_task_body)
}

pub fn patched_body(
    max_size: usize,
) -> impl Filter<Extract = (PatchedTask,), Error = Rejection> + Copy {
    warp::path::full()
        .and(warp::method())
        .and(warp::body::bytes())
        .and(warp::any().map(move || max_size))
        .and_then(parse_patched_task_body)
}

#[instrument(skip_all)]
async fn parse_task_id(path: FullPath, method: Method) -> Result<Uuid, Rejection> {
    let path = path.as_str();
    match path_to_uuid(path) {
        Ok(id) => Ok(id),
        Err(()) => {
            warn!(%method, %path, "Invalid path");
            Err(warp::reject::custom(AppError::InvalidPath))
        }
    }
}

#[instrument(skip_all)]
async fn parse_task_body(
    path: FullPath,
    method: Method,
    body: Bytes,
    max_size: usize,
) -> Result<Task, Rejection> {
    let path = path.as_str();
    deserialize::<Task>(&body, &method, path, max_size).map_err(warp::reject::custom)
}

#[instrument(skip_all)]
async fn parse_patched_task_body(
    path: FullPath,
    method: Method,
    body: Bytes,
    max_size: usize,
) -> Result<PatchedTask, Rejection> {
    let path = path.as_str();
    deserialize::<PatchedTask>(&body, &method, path, max_size).map_err(warp::reject::custom)
}

/// Deserializes a JSON body into a type T
#[instrument(skip_all)]
fn deserialize<T: DeserializeOwned>(
    body: &[u8],
    method: &Method,
    path: &str,
    max_size: usize,
) -> Result<T, AppError> {
    if body.len() > max_size {
        warn!(%method, %path, "Invalid body size");
        return Err(AppError::InvalidBodySize);
    }

    match serde_json::from_slice(body) {
        Ok(value) => Ok(value),
        Err(_) => {
            warn!(%method, %path, "Invalid body JSON");

            Err(AppError::InvalidJsonBody)
        }
    }
}

/// Takes a path and creates a UUID
fn path_to_uuid(path: &str) -> Result<Uuid, ()> {
    // Path should start with a slash
    let id = path.strip_prefix('/').ok_or(())?;

    // Next entry should be the UUID and thus no more slashes
    if id.is_empty() || id.contains('/') {
        return Err(());
    }

    Uuid::parse_str(id).map_err(|_| ())
}
