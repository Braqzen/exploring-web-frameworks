use rocket::{
    Data, Request,
    fairing::{Fairing, Info, Kind},
};
use tracing::{debug, instrument};

pub struct LogFairing;

#[rocket::async_trait]
impl Fairing for LogFairing {
    fn info(&self) -> Info {
        Info {
            name: "Log",
            kind: Kind::Request,
        }
    }

    #[instrument(skip_all)]
    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let method = req.method();
        let path = req.uri();

        debug!(%method, %path, "Incoming request");
    }
}
