use crate::routes::errors::AppError;
use rand::{RngExt, rng};
use std::time::Duration;
use tokio::time::sleep;
use warp::{Filter, reject::Rejection};

pub fn chaos_filter() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::any().and_then(chaos_impl).untuple_one()
}

async fn chaos_impl() -> Result<(), Rejection> {
    if rng().random_range(0..=100) < 5 {
        let duration = Duration::from_micros(rng().random_range(500..=1500));
        sleep(duration).await;
    }
    if rng().random_range(0..=100) < 5 {
        return Err(warp::reject::custom(AppError::Internal));
    }
    Ok(())
}
