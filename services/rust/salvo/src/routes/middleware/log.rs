use salvo::{Depot, FlowCtrl, Request, Response};
use tracing::debug;

#[salvo::handler]
pub async fn log_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    debug!(method = %req.method(), path = %req.uri(), "Incoming request");

    ctrl.call_next(req, depot, res).await;
}
