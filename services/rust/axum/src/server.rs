use crate::{router::router, state::State};
use axum::serve;
use eyre::Result;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{
    net::TcpListener,
    signal::unix::{SignalKind, signal},
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
        // Handle running locally and interrupting the process with ctrl+c.º
        let mut sigint = signal(SignalKind::interrupt())?;

        // Handle running in a container and terminating the process with docker stop.
        let mut sigterm = signal(SignalKind::terminate())?;

        let listener = TcpListener::bind(self.socket).await?;

        info!(socket = self.socket.to_string(), "Starting router");

        serve(listener, router(self.state))
            .with_graceful_shutdown(async move {
                tokio::select! {
                    _ = sigint.recv() => info!("Received interrupt signal"),
                    _ = sigterm.recv() => info!("Received terminate signal"),
                }
            })
            .await?;

        Ok(())
    }
}
