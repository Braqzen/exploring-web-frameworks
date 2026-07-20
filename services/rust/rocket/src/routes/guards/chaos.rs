use crate::routes::errors::AppError;
use app::state::AppState;
use rand::{RngExt, rng};
use rocket::{
    Request,
    http::Status,
    request::{FromRequest, Outcome},
    tokio::time::sleep,
};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

pub struct ChaosGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ChaosGuard {
    type Error = AppError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(state) = req.rocket().state::<Arc<Mutex<AppState>>>() else {
            tracing::error!(
                method = %req.method(),
                path = %req.uri(),
                "Missing app state in ChaosGuard"
            );
            return Outcome::Error((Status::InternalServerError, AppError::Internal));
        };

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
                    "Poisoned lock in ChaosGuard"
                );
                return Outcome::Error((Status::InternalServerError, AppError::Internal));
            }
        };

        if latency_enabled && rng().random_range(0..=100) < latency_rate {
            let duration = Duration::from_micros(rng().random_range(500..=1500));
            sleep(duration).await;
        }

        if error_enabled && rng().random_range(0..=100) < error_rate {
            return Outcome::Error((Status::InternalServerError, AppError::Internal));
        }

        Outcome::Success(Self)
    }
}
