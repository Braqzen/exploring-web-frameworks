use app::state::AppState;
use rand::{RngExt, rng};
use salvo::{Depot, FlowCtrl, Request, Response, http::StatusCode, writing::Json};
use serde_json::json;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::sleep;

#[salvo::handler]
pub async fn chaos_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let Ok(state) = depot.obtain::<Arc<Mutex<AppState>>>().cloned() else {
        tracing::error!(
            method = %req.method(),
            path = %req.uri(),
            "Missing app state in chaos_middleware"
        );
        res.stuff(
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Internal server error"})),
        );
        return;
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
                "Poisoned lock in chaos_middleware"
            );
            res.stuff(
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal server error"})),
            );
            return;
        }
    };

    if latency_enabled && rng().random_range(0..=100) < latency_rate {
        let duration = Duration::from_micros(rng().random_range(500..=1500));
        sleep(duration).await;
    }

    if error_enabled && rng().random_range(0..=100) < error_rate {
        res.stuff(
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Internal server error"})),
        );
        return;
    }

    ctrl.call_next(req, depot, res).await;
}
