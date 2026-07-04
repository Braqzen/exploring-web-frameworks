use crate::routes::errors::AppError;
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use rand::{RngExt, rng};
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

#[instrument(skip_all)]
pub async fn chaos_middleware(req: Request, next: Next) -> Response {
    if rng().random_range(0..=100) < 5 {
        let duration = Duration::from_micros(rng().random_range(500..=1500));
        sleep(duration).await;
    }
    if rng().random_range(0..=100) < 5 {
        return AppError::Internal.into_response();
    }

    next.run(req).await
}
