use crate::routes::{
    middleware::{chaos_middleware, log_middleware},
    router,
};
use actix_web::{App, HttpServer, middleware::from_fn, web::Data};
use app::{config::AppConfig, state::AppState};
use eyre::Result;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tracing::info;

pub struct Server {
    socket: SocketAddr,
    state: Arc<Mutex<AppState>>,
}

impl Server {
    pub fn new(socket: SocketAddr, app_config: AppConfig) -> Self {
        let state = AppState::new(app_config);
        Self {
            socket,
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub async fn run(self) -> Result<()> {
        info!(socket = self.socket.to_string(), "Starting router");

        HttpServer::new(move || {
            App::new()
                .wrap(from_fn(chaos_middleware))
                .wrap(from_fn(log_middleware))
                .app_data(Data::from(self.state.clone()))
                .configure(|cfg| router(cfg, self.state.clone()))
        })
        .shutdown_timeout(2)
        .bind(self.socket)?
        .run()
        .await?;

        Ok(())
    }
}
