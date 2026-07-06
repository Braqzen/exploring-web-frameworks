use actix_web::{
    Error,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};
use tracing::{debug, instrument};

#[instrument(skip_all)]
pub async fn log_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let method = req.method();
    let path = req.uri();

    debug!(%method, %path, "Incoming request");

    next.call(req).await
}
