use crate::{api::middleware::validate_request, router::router, state::State};
use actix_web::{App, HttpServer, middleware::from_fn, web::Data};
use eyre::Result;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tracing::info;

pub struct Server {
    socket: SocketAddr,
    state: Arc<Mutex<State>>,
}

impl Server {
    pub fn new(socket: SocketAddr) -> Self {
        Self {
            socket,
            state: Arc::new(Mutex::new(State::new())),
        }
    }

    pub async fn run(self) -> Result<()> {
        info!(socket = self.socket.to_string(), "Starting router");

        HttpServer::new(move || {
            App::new()
                .wrap(from_fn(validate_request))
                .app_data(Data::from(self.state.clone()))
                .configure(router)
        })
        .shutdown_timeout(2)
        .bind(self.socket)?
        .run()
        .await?;

        Ok(())
    }
}
