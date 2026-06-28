use crate::{router::router, state::State};
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
        let listener = TcpListener::bind(self.socket);

        let app = router(self.state);

        // Handle running locally and interrupting the process with ctrl+c.º
        let mut sigint = signal(SignalKind::interrupt())?;

        // Handle running in a container and terminating the process with docker stop.
        let mut sigterm = signal(SignalKind::terminate())?;

        info!(socket = self.socket.to_string(), "Starting router");

        PoemServer::new(listener)
            .run_with_graceful_shutdown(
                app,
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
