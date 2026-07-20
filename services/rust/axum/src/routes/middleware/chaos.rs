use crate::routes::errors::AppError;
use app::state::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use rand::{RngExt, rng};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::sleep;
use tracing::instrument;

#[instrument(skip_all)]
pub async fn chaos_middleware(
    State(state): State<Arc<Mutex<AppState>>>,
    req: Request,
    next: Next,
) -> Response {
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
            return AppError::Internal.into_response();
        }
    };

    if latency_enabled && rng().random_range(0..=100) < latency_rate {
        let duration = Duration::from_micros(rng().random_range(500..=1500));
        sleep(duration).await;
    }

    if error_enabled && rng().random_range(0..=100) < error_rate {
        return AppError::Internal.into_response();
    }

    next.run(req).await
}
