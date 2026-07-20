use axum::{extract::Request, middleware::Next, response::Response};
use tracing::{debug, instrument};

#[instrument(skip_all)]
pub async fn log_middleware(req: Request, next: Next) -> Response {
    debug!(method = %req.method(), path = %req.uri(), "Incoming request");

    next.run(req).await
}
