use crate::task::{PatchedTask, Task};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::convert::Infallible;
use tracing::{instrument, warn};
use uuid::Uuid;
use warp::{
    Filter,
    filters::path::FullPath,
    http::{Method, StatusCode},
    hyper::body::Bytes,
    reject::Rejection,
    reply::{Reply, Response, json, with_status},
};

// TODO: Note this is a magic number
// TODO: Make configurable?
/// The maximum size of a request body in bytes (64KB)
const MAX_BODY_SIZE: usize = 1024 * 64;

#[derive(Debug)]
pub struct ValidationError {
    status: StatusCode,
    message: &'static str,
}
impl warp::reject::Reject for ValidationError {}

/// Wrapper creating UUID or returning an error response
pub fn task_id() -> impl Filter<Extract = (Uuid,), Error = Rejection> + Copy {
    warp::path::full()
        .and(warp::method())
        .and_then(parse_task_id)
}

pub fn task_body() -> impl Filter<Extract = (Task,), Error = Rejection> + Copy {
    warp::path::full()
        .and(warp::method())
        .and(warp::body::bytes())
        .and_then(parse_task_body)
}

pub async fn handle_rejection(err: Rejection) -> Result<Response, Infallible> {
    if let Some(ValidationError { status, message }) = err.find() {
        return Ok(json_error(*status, message));
    }
    if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        return Ok(json_error(
            StatusCode::METHOD_NOT_ALLOWED,
            "Method not allowed",
        ));
    }
    if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        return Ok(json_error(
            StatusCode::PAYLOAD_TOO_LARGE,
            "Invalid body size",
        ));
    }
    Ok(json_error(StatusCode::NOT_FOUND, "Invalid path"))
}

pub fn patched_body() -> impl Filter<Extract = (PatchedTask,), Error = Rejection> + Copy {
    warp::path::full()
        .and(warp::method())
        .and(warp::body::bytes())
        .and_then(parse_patched_task_body)
}

#[instrument(skip_all)]
async fn parse_task_id(path: FullPath, method: Method) -> Result<Uuid, Rejection> {
    let path = path.as_str();
    match path_to_uuid(path) {
        Ok(id) => Ok(id),
        Err(()) => {
            warn!(%method, %path, "Invalid path");
            Err(warp::reject::custom(ValidationError {
                status: StatusCode::NOT_FOUND,
                message: "Invalid path",
            }))
        }
    }
}

#[instrument(skip_all)]
async fn parse_task_body(path: FullPath, method: Method, body: Bytes) -> Result<Task, Rejection> {
    let path = path.as_str();
    deserialize::<Task>(&body, &method, path).map_err(warp::reject::custom)
}

#[instrument(skip_all)]
async fn parse_patched_task_body(
    path: FullPath,
    method: Method,
    body: Bytes,
) -> Result<PatchedTask, Rejection> {
    let path = path.as_str();
    deserialize::<PatchedTask>(&body, &method, path).map_err(warp::reject::custom)
}

/// Deserializes a JSON body into a type T
#[instrument(skip_all)]
fn deserialize<T: DeserializeOwned>(
    body: &[u8],
    method: &Method,
    path: &str,
) -> Result<T, ValidationError> {
    if body.len() > MAX_BODY_SIZE {
        warn!(%method, %path, "Invalid body size");
        return Err(ValidationError {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            message: "Invalid body size",
        });
    }

    match serde_json::from_slice(body) {
        Ok(value) => Ok(value),
        Err(err) => {
            warn!(%method, %path, "Invalid body JSON");
            if err.is_syntax() {
                return Err(ValidationError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid body JSON",
                });
            }

            Err(ValidationError {
                status: StatusCode::UNPROCESSABLE_ENTITY,
                message: "Invalid body JSON",
            })
        }
    }
}

fn json_error(status: StatusCode, message: &str) -> Response {
    with_status(json(&json!({ "error": message })), status).into_response()
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
