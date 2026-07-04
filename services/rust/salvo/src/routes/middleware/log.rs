use salvo::{Depot, FlowCtrl, Request, Response};
use tracing::debug;

#[salvo::handler]
pub async fn log_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let method = req.method();
    let path = req.uri();

    debug!(%method, %path, "Incoming request");

    ctrl.call_next(req, depot, res).await;
}
