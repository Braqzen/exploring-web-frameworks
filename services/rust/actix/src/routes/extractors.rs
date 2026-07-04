use crate::routes::errors::AppError;
use actix_web::{FromRequest, HttpRequest, dev::Payload, web::Json};
use serde::de::DeserializeOwned;
use std::{
    future::{Ready, ready},
    pin::Pin,
};
use uuid::Uuid;

pub struct AppPath(pub Uuid);

impl FromRequest for AppPath {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let result = req
            .match_info()
            .get("task_id")
            .ok_or(AppError::InvalidPath)
            .and_then(|s| Uuid::parse_str(s).map_err(|_| AppError::InvalidPath))
            .map(AppPath);

        ready(result)
    }
}

pub struct AppJson<T>(pub T);

impl<T> FromRequest for AppJson<T>
where
    T: DeserializeOwned + 'static,
{
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = Json::<T>::from_request(req, payload);
        Box::pin(async move {
            fut.await
                .map(|json| AppJson(json.into_inner()))
                .map_err(|_| AppError::InvalidJsonBody)
        })
    }
}
