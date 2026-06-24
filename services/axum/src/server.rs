use crate::{
    api::handlers::{
        delete::remove, get::fetch, patch::partial_update, post::insert, put::overwrite,
    },
    task::Task,
};
use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    serve,
};
use eyre::Result;
use opentelemetry::{global, trace::FutureExt};
use opentelemetry_http::HeaderExtractor;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tracing::info;
use uuid::Uuid;

// Associate the trace ID from the generator to each request handler.
async fn trace_context(request: Request, next: Next) -> Response {
    let parent =
        global::get_text_map_propagator(|prop| prop.extract(&HeaderExtractor(request.headers())));
    next.run(request).with_context(parent).await
}

pub struct Server {
    socket: SocketAddr,
    state: Arc<Mutex<State>>,
}

pub struct State {
    pub tasks: HashMap<Uuid, Task>,
}

impl State {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }
}

impl Server {
    pub async fn new(socket: SocketAddr) -> Result<Self> {
        Ok(Self {
            socket,
            state: Arc::new(Mutex::new(State::new())),
        })
    }

    pub async fn run(self) -> Result<()> {
        let listener = TcpListener::bind(self.socket).await?;

        let app = Router::new()
            .route("/", post(insert))
            .route(
                "/{task_id}",
                get(fetch)
                    .put(overwrite)
                    .patch(partial_update)
                    .delete(remove),
            )
            .layer(middleware::from_fn(trace_context))
            .with_state(self.state);

        info!(socket = self.socket.to_string(), "Starting router");

        serve(listener, app).await?;

        Ok(())
    }
}
