use poem::{Endpoint, IntoResponse, Request, Response, Result};
use tracing::{debug, instrument};

#[instrument(skip_all)]
pub async fn log_middleware<E: Endpoint>(next: E, req: Request) -> Result<Response> {
    let method = req.method();
    let path = req.uri();

    debug!(%method, %path, "Incoming request");

    next.call(req).await.map(IntoResponse::into_response)
}
