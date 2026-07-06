use crate::routes::errors::AppError;
use salvo::{Depot, Extractible, Request, Writer, extract::Metadata};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use uuid::Uuid;

pub struct AppPath {
    pub task_id: Uuid,
}

impl<'ex> Extractible<'ex> for AppPath {
    fn metadata() -> &'static Metadata {
        static METADATA: Metadata = Metadata::new("AppPath");
        &METADATA
    }

    fn extract(
        req: &'ex mut Request,
        _depot: &'ex mut Depot,
    ) -> impl Future<Output = Result<Self, impl Writer + Send + Debug + 'static>> + Send
    where
        Self: Sized,
    {
        async move {
            let task_id = req.param::<Uuid>("task_id").ok_or(AppError::InvalidPath)?;

            Ok::<Self, AppError>(Self { task_id })
        }
    }
}

pub struct AppJson<T> {
    pub value: T,
}

impl<'ex, T> Extractible<'ex> for AppJson<T>
where
    T: DeserializeOwned + Send + 'static,
{
    fn metadata() -> &'static Metadata {
        static METADATA: Metadata = Metadata::new("AppJson");
        &METADATA
    }

    fn extract(
        req: &'ex mut Request,
        _depot: &'ex mut Depot,
    ) -> impl Future<Output = Result<Self, impl Writer + Send + Debug + 'static>> + Send
    where
        Self: Sized,
    {
        async move {
            req.parse_json::<T>()
                .await
                .map(|value| Self { value })
                .map_err(|_| AppError::InvalidJsonBody)
        }
    }
}
