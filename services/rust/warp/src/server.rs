use crate::routes::router;
use app::state::AppState;
use eyre::Result;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::signal::unix::{SignalKind, signal};
use tracing::info;

pub struct Server {
    socket: SocketAddr,
    state: Arc<Mutex<AppState>>,
}

impl Server {
    pub fn new(socket: SocketAddr) -> Self {
        Self {
            socket,
            state: Arc::new(Mutex::new(AppState::new())),
        }
    }

    pub async fn run(self) -> Result<()> {
        // Handle running locally and interrupting the process with ctrl+c.º
        let mut sigint = signal(SignalKind::interrupt())?;

        // Handle running in a container and terminating the process with docker stop.
        let mut sigterm = signal(SignalKind::terminate())?;

        info!(socket = self.socket.to_string(), "Starting router");

        warp::serve(router(self.state))
            .bind(self.socket)
            .await
            .graceful(async move {
                tokio::select! {
                    _ = sigint.recv() => info!("Received interrupt signal"),
                    _ = sigterm.recv() => info!("Received terminate signal"),
                }
            })
            .run()
            .await;

        Ok(())
    }
}
