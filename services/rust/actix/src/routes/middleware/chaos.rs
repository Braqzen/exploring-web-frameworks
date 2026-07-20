use crate::routes::errors::AppError;
use actix_web::{
    Error, ResponseError,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    rt::time::sleep,
    web::Data,
};
use app::state::AppState;
use rand::{RngExt, rng};
use std::{sync::Mutex, time::Duration};
use tracing::instrument;

#[instrument(skip_all)]
pub async fn chaos_middleware(
    state: Data<Mutex<AppState>>,
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let (latency_enabled, latency_rate, error_enabled, error_rate) = match state.lock() {
        Ok(guard) => (
            guard.config.latency.enabled,
            guard.config.latency.rate,
            guard.config.error.enabled,
            guard.config.error.rate,
        ),
        Err(_) => {
            tracing::error!(
                method = %req.method(),
                path = %req.uri(),
                "Poisoned lock in chaos_middleware"
            );
            return Ok(req.into_response(AppError::Internal.error_response()));
        }
    };

    if latency_enabled && rng().random_range(0..=100) < latency_rate {
        sleep(Duration::from_micros(rng().random_range(500..=1500))).await;
    }

    if error_enabled && rng().random_range(0..=100) < error_rate {
        return Ok(req.into_response(AppError::Internal.error_response()));
    }

    next.call(req).await
}
