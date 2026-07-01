use crate::task::{PatchedTask, Task};
use actix_web::{
    Error, HttpMessage, HttpRequest, HttpResponse,
    body::{self, BodyStream, BoxBody},
    dev::{Payload, ServiceRequest, ServiceResponse},
    http::{
        Method,
        header::{self, HeaderValue},
    },
    middleware::Next,
    rt::time::sleep,
    web::Bytes,
};
use rand::{RngExt, rng};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::time::Duration;
use tracing::{instrument, warn};
use uuid::Uuid;

// TODO: Note this is a magic number
// TODO: Make configurable?
/// The maximum size of a request body in bytes (64KB)
const MAX_BODY_SIZE: usize = 1024 * 64;

#[instrument(skip_all)]
pub async fn validate_request(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    if rng().random_range(0..=100) < 5 {
        sleep(Duration::from_micros(rng().random_range(500..=1500))).await;
    }
    if rng().random_range(0..=100) < 5 {
        return Ok(req.into_response(
            HttpResponse::InternalServerError().json(json!({"error": "Internal server error"})),
        ));
    }

    let method = req.method().clone();
    let path = req.path().to_string();

    match method {
        Method::POST => {
            // Only posts to the root are allowed
            if path != "/" {
                warn!(%method, %path, "Invalid path");
                return Ok(req.into_response(
                    HttpResponse::NotFound().json(json!({"error": "Invalid path"})),
                ));
            }

            let (request, payload) = req.into_parts();

            let bytes = match body_to_bytes(payload, &method, &path).await {
                Ok(bytes) => bytes,
                Err(response) => return Ok(reject(request, response)),
            };

            let task = match deserialize::<Task>(&bytes, &method, &path) {
                Ok(task) => task,
                Err(response) => return Ok(reject(request, response)),
            };

            request.extensions_mut().insert(task);

            next.call(service_request(request)).await
        }
        Method::PUT => {
            let id = match task_id(&path, &method) {
                Ok(id) => id,
                Err(response) => return Ok(req.into_response(response)),
            };

            let (request, payload) = req.into_parts();

            let bytes = match body_to_bytes(payload, &method, &path).await {
                Ok(bytes) => bytes,
                Err(response) => return Ok(reject(request, response)),
            };

            let task = match deserialize::<Task>(&bytes, &method, &path) {
                Ok(task) => task,
                Err(response) => return Ok(reject(request, response)),
            };

            request.extensions_mut().insert(task);
            request.extensions_mut().insert(id);

            next.call(service_request(request)).await
        }
        Method::PATCH => {
            let id = match task_id(&path, &method) {
                Ok(id) => id,
                Err(response) => return Ok(req.into_response(response)),
            };

            let (request, payload) = req.into_parts();

            let bytes = match body_to_bytes(payload, &method, &path).await {
                Ok(bytes) => bytes,
                Err(response) => return Ok(reject(request, response)),
            };

            let task = match deserialize::<PatchedTask>(&bytes, &method, &path) {
                Ok(task) => task,
                Err(response) => return Ok(reject(request, response)),
            };

            request.extensions_mut().insert(task);
            request.extensions_mut().insert(id);

            next.call(service_request(request)).await
        }
        Method::DELETE => {
            let id = match task_id(&path, &method) {
                Ok(id) => id,
                Err(response) => return Ok(req.into_response(response)),
            };

            req.extensions_mut().insert(id);

            next.call(req).await
        }
        Method::GET => {
            let id = match task_id(&path, &method) {
                Ok(id) => id,
                Err(response) => return Ok(req.into_response(response)),
            };

            req.extensions_mut().insert(id);

            next.call(req).await
        }
        _ => {
            warn!(%method, %path, "Method not allowed");
            let allow = if path == "/" {
                "POST"
            } else {
                "GET, PUT, PATCH, DELETE"
            };

            return Ok(req.into_response(
                HttpResponse::MethodNotAllowed()
                    .append_header((header::ALLOW, HeaderValue::from_static(allow)))
                    .json(json!({"error": "Method not allowed"})),
            ));
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
fn task_id(path: &str, method: &Method) -> Result<Uuid, HttpResponse> {
    match path_to_uuid(path) {
        Ok(id) => Ok(id),
        Err(()) => {
            warn!(%method, %path, "Invalid path");
            Err(HttpResponse::NotFound().json(json!({"error": "Invalid path"})))
        }
    }
}

/// Converts the body of a request into bytes for deserialization into types
async fn body_to_bytes(
    payload: Payload,
    method: &Method,
    path: &str,
) -> Result<Bytes, HttpResponse> {
    let stream = BodyStream::new(payload);

    match body::to_bytes_limited(stream, MAX_BODY_SIZE).await {
        Ok(Ok(bytes)) => Ok(bytes),
        Ok(Err(_)) => {
            warn!(%method, %path, "Invalid body");
            Err(HttpResponse::BadRequest().json(json!({"error": "Invalid body"})))
        }
        Err(_) => {
            warn!(%method, %path, "Invalid body size");
            Err(HttpResponse::PayloadTooLarge().json(json!({"error": "Invalid body size"})))
        }
    }
}

/// Deserializes a JSON body into a type T
fn deserialize<T: DeserializeOwned>(
    bytes: &[u8],
    method: &Method,
    path: &str,
) -> Result<T, HttpResponse> {
    match serde_json::from_slice::<T>(&bytes) {
        Ok(value) => Ok(value),
        Err(error) => {
            warn!(%method, %path, "Invalid body JSON");

            if error.is_syntax() {
                return Err(HttpResponse::BadRequest().json(json!({"error": "Invalid body JSON"})));
            }
            Err(HttpResponse::UnprocessableEntity().json(json!({"error": "Invalid body JSON"})))
        }
    }
}

fn service_request(request: HttpRequest) -> ServiceRequest {
    ServiceRequest::from_parts(request, Payload::None)
}

fn reject(request: HttpRequest, response: HttpResponse) -> ServiceResponse {
    service_request(request).into_response(response)
}
