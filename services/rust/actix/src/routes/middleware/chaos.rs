use crate::routes::errors::AppError;
use actix_web::{
    Error, ResponseError,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    rt::time::sleep,
};
use rand::{RngExt, rng};
use std::time::Duration;
use tracing::instrument;

#[instrument(skip_all)]
pub async fn chaos_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    if rng().random_range(0..=100) < 5 {
        sleep(Duration::from_micros(rng().random_range(500..=1500))).await;
    }
    if rng().random_range(0..=100) < 5 {
        return Ok(req.into_response(AppError::Internal.error_response()));
    }

    next.call(req).await
}
