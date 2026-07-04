use crate::{
    routes::{invalid_method_handler, invalid_path_handler, router},
    state::AppState,
};
use eyre::Result;
use salvo::{Listener, Server as SalvoServer, Service, catcher::Catcher, prelude::TcpListener};
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
        let listener = TcpListener::new(self.socket).bind().await;

        // Handle running locally and interrupting the process with ctrl+c.º
        let mut sigint = signal(SignalKind::interrupt())?;

        // Handle running in a container and terminating the process with docker stop.
        let mut sigterm = signal(SignalKind::terminate())?;

        info!(socket = self.socket.to_string(), "Starting router");

        let service = Service::new(router(self.state)).catcher(
            Catcher::default()
                .hoop(invalid_path_handler)
                .hoop(invalid_method_handler),
        );

        let server = SalvoServer::new(listener);
        let handle = server.handle();

        tokio::spawn(async move {
            tokio::select! {
                _ = sigint.recv() => info!("Received interrupt signal"),
                _ = sigterm.recv() => info!("Received terminate signal"),
            }

            handle.stop_graceful(None);
        });

        server.serve(service).await;

        Ok(())
    }
}
