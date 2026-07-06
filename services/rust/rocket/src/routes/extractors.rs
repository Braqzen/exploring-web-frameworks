use crate::routes::errors::AppError;
use rocket::{
    Data, Request,
    data::{FromData, Outcome},
    request::FromParam,
    serde::json::Json,
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use tracing::warn;
use uuid::Uuid;

pub struct AppPath(pub Uuid);

impl<'r> FromParam<'r> for AppPath {
    type Error = &'r str;
    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param).map(AppPath).map_err(|_| param)
    }
}

impl AppPath {
    pub fn into_inner(&self) -> Uuid {
        self.0
    }
}

pub struct AppJson<T>(pub T);

impl<T> AppJson<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[rocket::async_trait]
impl<'r, T: DeserializeOwned> FromData<'r> for AppJson<T> {
    type Error = AppError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let method = req.method();
        let path = req.uri().path();

        if !req
            .content_type()
            .is_some_and(|content_type| content_type.is_json())
        {
            warn!(%method, %path, "Invalid body JSON");
            return fail(req, AppError::InvalidJsonBody);
        }

        let limit = req.limits().get("json").expect("json limit not configured");

        let bytes = match data.open(limit).into_bytes().await {
            Ok(capped) if capped.is_complete() => capped.into_inner(),
            Ok(_) | Err(_) => {
                warn!(%method, %path, "Invalid body JSON");
                return fail(req, AppError::InvalidJsonBody);
            }
        };

        match serde_json::from_slice::<T>(&bytes) {
            Ok(value) => Outcome::Success(AppJson(value)),
            Err(_) => {
                warn!(%method, %path, "Invalid body JSON");
                fail(req, AppError::InvalidJsonBody)
            }
        }
    }
}

// Catch our type so we can differentiate in the catchers
fn fail<'r, T: DeserializeOwned>(req: &'r Request<'_>, err: AppError) -> Outcome<'r, AppJson<T>> {
    let _ = req.local_cache(|| err.clone());
    Outcome::Error((err.clone().status(), err))
}

pub fn error_json(req: &Request) -> Json<Value> {
    // Ignores the enum variant
    let err = req.local_cache(|| AppError::InvalidJsonBody);
    Json(json!({"error": err.to_owned().message()}))
}
