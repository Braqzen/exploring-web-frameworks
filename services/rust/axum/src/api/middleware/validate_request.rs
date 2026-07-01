use std::time::Duration;

use crate::task::{PatchedTask, Task};
use axum::{
    Json,
    body::{self, Body, Bytes},
    extract::Request,
    http::{HeaderValue, Method, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use rand::{RngExt, rng};
use serde::de::DeserializeOwned;
use serde_json::json;
use tokio::time::sleep;
use tracing::{instrument, warn};
use uuid::Uuid;

// TODO: Note this is a magic number
// TODO: Make configurable?
/// The maximum size of a request body in bytes (64KB)
const MAX_BODY_SIZE: usize = 1024 * 64;

#[instrument(skip_all)]
pub async fn validate_request(req: Request, next: Next) -> Response {
    if rng().random_range(0..=100) < 5 {
        let duration = Duration::from_micros(rng().random_range(500..=1500));
        sleep(duration).await;
    }
    if rng().random_range(0..=100) < 5 {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Internal server error"})),
        )
            .into_response();
    }

    let (mut parts, request_body) = req.into_parts();
    let method = parts.method.clone();
    let path = parts.uri.path();

    match method {
        Method::POST => {
            // Only posts to the root are allowed
            if path != "/" {
                warn!(%method, %path, "Invalid path");
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Invalid path"})),
                )
                    .into_response();
            }

            let bytes = match body_to_bytes(request_body, &method, path).await {
                Ok(bytes) => bytes,
                Err(response) => return response,
            };

            let task = match deserialize::<Task>(&bytes, &method, path) {
                Ok(task) => task,
                Err(response) => return response,
            };

            parts.extensions.insert(task);

            next.run(Request::from_parts(parts, Body::empty())).await
        }
        Method::PUT => {
            let id = match task_id(path, &method) {
                Ok(id) => id,
                Err(response) => return response,
            };

            let bytes = match body_to_bytes(request_body, &method, path).await {
                Ok(bytes) => bytes,
                Err(response) => return response,
            };

            let task = match deserialize::<Task>(&bytes, &method, path) {
                Ok(task) => task,
                Err(response) => return response,
            };

            parts.extensions.insert(task);
            parts.extensions.insert(id);

            next.run(Request::from_parts(parts, Body::empty())).await
        }
        Method::PATCH => {
            let id = match task_id(path, &method) {
                Ok(id) => id,
                Err(response) => return response,
            };

            let bytes = match body_to_bytes(request_body, &method, path).await {
                Ok(bytes) => bytes,
                Err(response) => return response,
            };

            let task = match deserialize::<PatchedTask>(&bytes, &method, path) {
                Ok(task) => task,
                Err(response) => return response,
            };

            parts.extensions.insert(task);
            parts.extensions.insert(id);

            next.run(Request::from_parts(parts, Body::empty())).await
        }
        Method::DELETE => {
            let id = match task_id(path, &method) {
                Ok(id) => id,
                Err(response) => return response,
            };

            parts.extensions.insert(id);

            next.run(Request::from_parts(parts, request_body)).await
        }
        Method::GET => {
            let id = match task_id(path, &method) {
                Ok(id) => id,
                Err(response) => return response,
            };

            parts.extensions.insert(id);

            next.run(Request::from_parts(parts, request_body)).await
        }
        _ => {
            warn!(%method, %path, "Method not allowed");
            let allow = if path == "/" {
                "POST"
            } else {
                "GET, PUT, PATCH, DELETE"
            };

            return (
                StatusCode::METHOD_NOT_ALLOWED,
                [(header::ALLOW, HeaderValue::from_static(allow))],
                Json(json!({"error": "Method not allowed"})),
            )
                .into_response();
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

/// Wrapper creating UUID or returning an error response
fn task_id(path: &str, method: &Method) -> Result<Uuid, Response> {
    match path_to_uuid(path) {
        Ok(id) => Ok(id),
        Err(()) => {
            warn!(%method, %path, "Invalid path");
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Invalid path"})),
            )
                .into_response());
        }
    }
}

/// Converts the body of a request into bytes for deserialization into types
async fn body_to_bytes(request_body: Body, method: &Method, path: &str) -> Result<Bytes, Response> {
    match body::to_bytes(request_body, MAX_BODY_SIZE).await {
        Ok(bytes) => Ok(bytes),
        Err(_) => {
            warn!(%method, %path, "Invalid body size");
            return Err((
                StatusCode::PAYLOAD_TOO_LARGE,
                Json(json!({"error": "Invalid body size"})),
            )
                .into_response());
        }
    }
}

/// Deserializes a JSON body into a type T
fn deserialize<T: DeserializeOwned>(
    bytes: &[u8],
    method: &Method,
    path: &str,
) -> Result<T, Response> {
    match serde_json::from_slice::<T>(&bytes) {
        Ok(value) => Ok(value),
        Err(error) => {
            warn!(%method, %path, "Invalid body JSON");

            if error.is_syntax() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Invalid body JSON"})),
                )
                    .into_response());
            }
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"error": "Invalid body JSON"})),
            )
                .into_response());
        }
    }
}
