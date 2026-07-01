use rand::{RngExt, rng};
use rocket::{
    Request,
    http::Status,
    request::{FromRequest, Outcome},
    tokio::time::sleep,
};
use std::time::Duration;

pub struct Chaos;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Chaos {
    type Error = ();

    async fn from_request(_: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if rng().random_range(0..=100) < 5 {
            let duration = Duration::from_micros(rng().random_range(500..=1500));
            sleep(duration).await;
        }
        if rng().random_range(0..=100) < 5 {
            return Outcome::Error((Status::InternalServerError, ()));
        }

        Outcome::Success(Chaos)
    }
}
