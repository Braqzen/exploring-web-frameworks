use salvo::{Depot, Request, Response, Writer, async_trait, http::StatusCode, writing::Json};
use serde_json::json;
use std::fmt::Display;

#[derive(Debug)]
pub enum AppError {
    TaskNotFound,
    Internal,
    InvalidPath,
    InvalidMethod,
    InvalidJsonBody,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::TaskNotFound => "Task not found",
            Self::Internal => "Internal server error",
            Self::InvalidPath => "Invalid path",
            Self::InvalidMethod => "Invalid method",
            Self::InvalidJsonBody => "Invalid body JSON",
        })
    }
}

impl AppError {
    pub fn render(self, res: &mut Response) {
        let status = match self {
            Self::TaskNotFound | Self::InvalidPath => StatusCode::NOT_FOUND,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidMethod => StatusCode::METHOD_NOT_ALLOWED,
            Self::InvalidJsonBody => StatusCode::UNPROCESSABLE_ENTITY,
        };
        res.stuff(status, Json(json!({"error": self.to_string()})));
    }
}

#[async_trait]
impl Writer for AppError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        self.render(res);
    }
}
