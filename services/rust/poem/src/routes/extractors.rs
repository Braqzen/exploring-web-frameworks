use crate::routes::errors::AppError;
use poem::{
    Error, FromRequest, Request, RequestBody, Result,
    web::{Json, Path},
};
use serde::de::DeserializeOwned;
use uuid::Uuid;

pub struct AppPath(pub Uuid);

pub struct AppJson<T>(pub T);

impl<'a> FromRequest<'a> for AppPath {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let Path(id) = Path::<Uuid>::from_request_without_body(req)
            .await
            .map_err(|_| Error::from(AppError::InvalidPath))?;

        Ok(AppPath(id))
    }
}

impl<'a, T: DeserializeOwned> FromRequest<'a> for AppJson<T> {
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self> {
        let Json(value) = Json::<T>::from_request(req, body)
            .await
            .map_err(|_| Error::from(AppError::InvalidJsonBody))?;

        Ok(AppJson(value))
    }
}
