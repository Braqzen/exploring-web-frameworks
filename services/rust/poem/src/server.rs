use crate::routes::router;
use app::{config::AppConfig, state::AppState};
use eyre::Result;
use poem::{Server as PoemServer, listener::TcpListener};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::signal::unix::{SignalKind, signal};
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
        let listener = TcpListener::bind(self.socket);

        // Handle running locally and interrupting the process with ctrl+c.º
        let mut sigint = signal(SignalKind::interrupt())?;

        // Handle running in a container and terminating the process with docker stop.
        let mut sigterm = signal(SignalKind::terminate())?;

        info!(socket = self.socket.to_string(), "Starting router");

        PoemServer::new(listener)
            .run_with_graceful_shutdown(
                router(self.state),
                async move {
                    tokio::select! {
                        _ = sigint.recv() => {
                            info!("Received interrupt signal");
                        }
                        _ = sigterm.recv() => {
                            info!("Received terminate signal");
                        }
                    }
                },
                Some(Duration::from_secs(2)),
            )
            .await?;

        Ok(())
    }
}
