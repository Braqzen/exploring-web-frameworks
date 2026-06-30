use rocket::{
    Data, Request,
    data::{FromData, Outcome},
    http::Status,
    serde::json::Json,
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use tracing::warn;

// TODO: Note this is a magic number
// TODO: Make configurable?
/// The maximum size of a request body in bytes (64KB)
const MAX_BODY_SIZE: usize = 1024 * 64;

pub struct Extract<T>(T);

impl<T> Extract<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[derive(Debug, Clone)]
pub enum BodyError {
    PayloadTooLarge,
    InvalidBody,
    InvalidJson,
}

impl BodyError {
    fn message(self) -> String {
        match self {
            Self::PayloadTooLarge => "Invalid body size",
            Self::InvalidBody => "Invalid body",
            Self::InvalidJson => "Invalid body JSON",
        }
        .to_string()
    }
}

#[rocket::async_trait]
impl<'r, T: DeserializeOwned> FromData<'r> for Extract<T> {
    type Error = BodyError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let method = req.method();
        let path = req.uri().path();

        let bytes = match data.open(MAX_BODY_SIZE.into()).into_bytes().await {
            Ok(capped) if capped.is_complete() => capped.into_inner(),
            Ok(_) => {
                warn!(%method, %path, "Invalid body size");
                return fail(req, Status::PayloadTooLarge, BodyError::PayloadTooLarge);
            }
            Err(_) => {
                warn!(%method, %path, "Invalid body");
                return fail(req, Status::BadRequest, BodyError::InvalidBody);
            }
        };

        match serde_json::from_slice::<T>(&bytes) {
            Ok(value) => Outcome::Success(Extract(value)),
            Err(error) if error.is_syntax() => {
                warn!(%method, %path, "Invalid body JSON");
                return fail(req, Status::BadRequest, BodyError::InvalidJson);
            }
            Err(_) => {
                warn!(%method, %path, "Invalid body JSON");
                return fail(req, Status::UnprocessableEntity, BodyError::InvalidJson);
            }
        }
    }
}

// Catch our type so we can differentiate in the catchers
fn fail<'r, T: DeserializeOwned>(
    req: &'r Request<'_>,
    status: Status,
    err: BodyError,
) -> Outcome<'r, Extract<T>> {
    let _ = req.local_cache(|| err.clone());
    Outcome::Error((status, err))
}

pub fn error_json(req: &Request) -> Json<Value> {
    // Ignores the enum variant
    let err = req.local_cache(|| BodyError::InvalidBody);
    Json(json!({"error": err.to_owned().message()}))
}
