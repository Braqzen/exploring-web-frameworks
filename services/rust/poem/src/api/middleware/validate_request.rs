use crate::task::{PatchedTask, Task};
use poem::{
    Body, Endpoint, IntoResponse, Request, Response, Result,
    error::ReadBodyError,
    http::{Method, StatusCode},
    web::Json,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use tracing::{instrument, warn};
use uuid::Uuid;

// TODO: Note this is a magic number
// TODO: Make configurable?
/// The maximum size of a request body in bytes (64KB)
const MAX_BODY_SIZE: usize = 1024 * 64;

#[instrument(skip_all)]
pub async fn validate_request<E: Endpoint>(next: E, mut req: Request) -> Result<Response> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    match method {
        Method::POST => {
            // Only posts to the root are allowed
            if path != "/" {
                warn!(%method, %path, "Invalid path");
                return Ok((
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Invalid path"})),
                )
                    .into_response());
            }

            let request_body = req.take_body();

            let bytes = match body_to_bytes(request_body, &method, &path).await {
                Ok(bytes) => bytes,
                Err(response) => return Ok(response),
            };

            let task = match deserialize::<Task>(&bytes, &method, &path) {
                Ok(task) => task,
                Err(response) => return Ok(response),
            };

            req.extensions_mut().insert(task);
            req.set_body(Body::empty());

            next.call(req).await.map(IntoResponse::into_response)
        }
        Method::PUT => {
            let id = match task_id(&path, &method) {
                Ok(id) => id,
                Err(response) => return Ok(response),
            };

            let request_body = req.take_body();

            let bytes = match body_to_bytes(request_body, &method, &path).await {
                Ok(bytes) => bytes,
                Err(response) => return Ok(response),
            };

            let task = match deserialize::<Task>(&bytes, &method, &path) {
                Ok(task) => task,
                Err(response) => return Ok(response),
            };

            req.extensions_mut().insert(task);
            req.extensions_mut().insert(id);
            req.set_body(Body::empty());

            next.call(req).await.map(IntoResponse::into_response)
        }
        Method::PATCH => {
            let id = match task_id(&path, &method) {
                Ok(id) => id,
                Err(response) => return Ok(response),
            };

            let request_body = req.take_body();

            let bytes = match body_to_bytes(request_body, &method, &path).await {
                Ok(bytes) => bytes,
                Err(response) => return Ok(response),
            };

            let task = match deserialize::<PatchedTask>(&bytes, &method, &path) {
                Ok(task) => task,
                Err(response) => return Ok(response),
            };

            req.extensions_mut().insert(task);
            req.extensions_mut().insert(id);
            req.set_body(Body::empty());

            next.call(req).await.map(IntoResponse::into_response)
        }
        Method::DELETE => {
            let id = match task_id(&path, &method) {
                Ok(id) => id,
                Err(response) => return Ok(response),
            };

            req.extensions_mut().insert(id);

            next.call(req).await.map(IntoResponse::into_response)
        }
        Method::GET => {
            let id = match task_id(&path, &method) {
                Ok(id) => id,
                Err(response) => return Ok(response),
            };

            req.extensions_mut().insert(id);

            next.call(req).await.map(IntoResponse::into_response)
        }
        _ => {
            warn!(%method, %path, "Method not allowed");
            Ok((
                StatusCode::METHOD_NOT_ALLOWED,
                Json(json!({"error": "Method not allowed"})),
            )
                .into_response())
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
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Invalid path"})),
            )
                .into_response())
        }
    }
}

/// Converts the body of a request into bytes for deserialization into types
async fn body_to_bytes(
    request_body: Body,
    method: &Method,
    path: &str,
) -> Result<Vec<u8>, Response> {
    match request_body.into_bytes_limit(MAX_BODY_SIZE).await {
        Ok(bytes) => Ok(bytes.to_vec()),
        Err(error) => match error {
            ReadBodyError::PayloadTooLarge => {
                warn!(%method, %path, "Invalid body size");
                Err((
                    StatusCode::PAYLOAD_TOO_LARGE,
                    Json(json!({"error": "Invalid body size"})),
                )
                    .into_response())
            }
            _ => {
                warn!(%method, %path, "Invalid body");
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Invalid body"})),
                )
                    .into_response())
            }
        },
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

            Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"error": "Invalid body JSON"})),
            )
                .into_response())
        }
    }
}
