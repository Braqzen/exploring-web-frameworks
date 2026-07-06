use std::convert::Infallible;
use tracing::{debug, instrument};
use warp::{Filter, filters::path::FullPath, http::Method};

pub fn log_filter() -> impl Filter<Extract = (), Error = Infallible> + Copy {
    warp::any()
        .and(warp::path::full())
        .and(warp::method())
        .map(log_request)
        .untuple_one()
}

#[instrument(skip_all)]
fn log_request(path: FullPath, method: Method) {
    debug!(%method, path = %path.as_str(), "Incoming request");
}
