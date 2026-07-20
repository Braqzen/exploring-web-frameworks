use crate::routes::errors::AppError;
use app::state::AppState;
use rand::{RngExt, rng};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::sleep;
use warp::{Filter, reject::Rejection};

pub fn chaos_filter(
    state: Arc<Mutex<AppState>>,
) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::any()
        .map(move || state.clone())
        .and_then(chaos_impl)
        .untuple_one()
}

async fn chaos_impl(state: Arc<Mutex<AppState>>) -> Result<(), Rejection> {
    let (latency_enabled, latency_rate, error_enabled, error_rate) = match state.lock() {
        Ok(guard) => (
            guard.config.latency.enabled,
            guard.config.latency.rate,
            guard.config.error.enabled,
            guard.config.error.rate,
        ),
        Err(_) => {
            return Err(warp::reject::custom(AppError::Internal));
        }
    };

    if latency_enabled && rng().random_range(0..=100) < latency_rate {
        let duration = Duration::from_micros(rng().random_range(500..=1500));
        sleep(duration).await;
    }

    if error_enabled && rng().random_range(0..=100) < error_rate {
        return Err(warp::reject::custom(AppError::Internal));
    }

    Ok(())
}
