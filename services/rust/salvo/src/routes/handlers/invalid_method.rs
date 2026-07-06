use crate::routes::errors::AppError;
use salvo::{FlowCtrl, Response, http::StatusCode};

#[salvo::handler]
pub async fn invalid_method_handler(res: &mut Response, ctrl: &mut FlowCtrl) {
    if res.status_code == Some(StatusCode::METHOD_NOT_ALLOWED) {
        AppError::InvalidMethod.render(res);
        ctrl.skip_rest();
    }
}
