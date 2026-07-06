use crate::routes::errors::AppError;
use salvo::{FlowCtrl, Response, http::StatusCode};

#[salvo::handler]
pub async fn invalid_path_handler(res: &mut Response, ctrl: &mut FlowCtrl) {
    if res.status_code == Some(StatusCode::NOT_FOUND) {
        AppError::InvalidPath.render(res);
        ctrl.skip_rest();
    }
}
