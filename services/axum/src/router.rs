use crate::{
    api::handlers::{
        delete::remove, get::fetch, patch::partial_update, post::insert, put::overwrite,
    },
    state::State,
};
use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};
use opentelemetry::{global, trace::FutureExt};
use opentelemetry_http::HeaderExtractor;
use std::sync::{Arc, Mutex};

pub fn router(state: Arc<Mutex<State>>) -> Router {
    Router::new()
        .route("/", post(insert))
        .route(
            "/{task_id}",
            get(fetch)
                .put(overwrite)
                .patch(partial_update)
                .delete(remove),
        )
        .layer(middleware::from_fn(trace_context))
        .with_state(state)
}

// Associate the trace ID from the generator to each request handler.
async fn trace_context(request: Request, next: Next) -> Response {
    let parent =
        global::get_text_map_propagator(|prop| prop.extract(&HeaderExtractor(request.headers())));
    next.run(request).with_context(parent).await
}
