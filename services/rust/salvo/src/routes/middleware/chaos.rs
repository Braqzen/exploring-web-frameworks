use rand::{RngExt, rng};
use salvo::{Depot, FlowCtrl, Request, Response, http::StatusCode, writing::Json};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[salvo::handler]
pub async fn chaos_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    if rng().random_range(0..=100) < 5 {
        let duration = Duration::from_micros(rng().random_range(500..=1500));
        sleep(duration).await;
    }
    if rng().random_range(0..=100) < 5 {
        res.stuff(
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Internal server error"})),
        );
        return;
    }

    ctrl.call_next(req, depot, res).await;
}
