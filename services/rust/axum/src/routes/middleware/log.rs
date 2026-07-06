use axum::{extract::Request, middleware::Next, response::Response};
use tracing::{debug, instrument};

#[instrument(skip_all)]
pub async fn log_middleware(req: Request, next: Next) -> Response {
    let method = req.method();
    let path = req.uri();

    debug!(%method, %path, "Incoming request");

    next.run(req).await
}
