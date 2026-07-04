use crate::routes::errors::AppError;
use poem::{Endpoint, IntoResponse, Request, Response, Result, error::ResponseError};
use rand::{RngExt, rng};
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

#[instrument(skip_all)]
pub async fn chaos_middleware<E: Endpoint>(next: E, req: Request) -> Result<Response> {
    if rng().random_range(0..=100) < 5 {
        let duration = Duration::from_micros(rng().random_range(500..=1500));
        sleep(duration).await;
    }
    if rng().random_range(0..=100) < 5 {
        return Ok(AppError::Internal.as_response());
    }

    next.call(req).await.map(IntoResponse::into_response)
}
