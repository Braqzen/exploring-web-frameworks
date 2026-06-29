use crate::task::{PatchedTask, Task};
use salvo::{
    Depot, FlowCtrl, Request, Response,
    http::{Method, ParseError, StatusCode},
    writing::Json,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use tracing::warn;
use uuid::Uuid;

// TODO: Note this is a magic number
// TODO: Make configurable?
/// The maximum size of a request body in bytes (64KB)
const MAX_BODY_SIZE: usize = 1024 * 64;

// macro does nonsense so fn name must be different than everywhere else
#[salvo::handler]
pub async fn validate_request_fn(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    match method {
        Method::POST => {
            // Only posts to the root are allowed
            if path != "/" {
                warn!(%method, %path, "Invalid path");
                res.stuff(
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Invalid path"})),
                );

                ctrl.skip_rest();
                return;
            }

            let task = match deserialize::<Task>(req, res, &method, &path).await {
                Ok(task) => task,
                Err(()) => {
                    ctrl.skip_rest();
                    return;
                }
            };

            depot.inject(task);

            ctrl.call_next(req, depot, res).await;
        }
        Method::PUT => {
            let id = match task_id(&path, &method, res) {
                Ok(id) => id,
                Err(()) => {
                    ctrl.skip_rest();
                    return;
                }
            };

            let task = match deserialize::<Task>(req, res, &method, &path).await {
                Ok(task) => task,
                Err(()) => {
                    ctrl.skip_rest();
                    return;
                }
            };

            depot.inject(task);
            depot.inject(id);

            ctrl.call_next(req, depot, res).await;
        }
        Method::PATCH => {
            let id = match task_id(&path, &method, res) {
                Ok(id) => id,
                Err(()) => {
                    ctrl.skip_rest();
                    return;
                }
            };

            let task = match deserialize::<PatchedTask>(req, res, &method, &path).await {
                Ok(task) => task,
                Err(()) => {
                    ctrl.skip_rest();
                    return;
                }
            };

            depot.inject(task);
            depot.inject(id);

            ctrl.call_next(req, depot, res).await;
        }
        Method::DELETE => {
            let id = match task_id(&path, &method, res) {
                Ok(id) => id,
                Err(()) => {
                    ctrl.skip_rest();
                    return;
                }
            };

            depot.inject(id);

            ctrl.call_next(req, depot, res).await;
        }
        Method::GET => {
            let id = match task_id(&path, &method, res) {
                Ok(id) => id,
                Err(()) => {
                    ctrl.skip_rest();
                    return;
                }
            };

            depot.inject(id);

            ctrl.call_next(req, depot, res).await;
        }
        _ => {
            warn!(%method, %path, "Method not allowed");
            res.stuff(
                StatusCode::METHOD_NOT_ALLOWED,
                Json(json!({"error": "Method not allowed"})),
            );
            ctrl.skip_rest();
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
fn task_id(path: &str, method: &Method, res: &mut Response) -> Result<Uuid, ()> {
    match path_to_uuid(path) {
        Ok(id) => Ok(id),
        Err(()) => {
            warn!(%method, %path, "Invalid path");
            res.stuff(
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Invalid path"})),
            );

            Err(())
        }
    }
}

/// Deserializes a JSON body into a type T
async fn deserialize<T: DeserializeOwned>(
    req: &mut Request,
    res: &mut Response,
    method: &Method,
    path: &str,
) -> Result<T, ()> {
    match req.parse_json_with_max_size::<T>(MAX_BODY_SIZE).await {
        Ok(value) => Ok(value),
        Err(error) => match error {
            ParseError::PayloadTooLarge => {
                warn!(%method, %path, "Invalid body size");
                res.stuff(
                    StatusCode::PAYLOAD_TOO_LARGE,
                    Json(json!({"error": "Invalid body size"})),
                );
                Err(())
            }
            ParseError::SerdeJson(error) if error.is_syntax() => {
                warn!(%method, %path, "Invalid body JSON");
                res.stuff(
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Invalid body JSON"})),
                );
                Err(())
            }
            ParseError::SerdeJson(_) => {
                warn!(%method, %path, "Invalid body JSON");
                res.stuff(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({"error": "Invalid body JSON"})),
                );
                Err(())
            }
            _ => {
                warn!(%method, %path, "Invalid body");
                res.stuff(
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Invalid body"})),
                );
                Err(())
            }
        },
    }
}
