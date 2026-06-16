use crate::api::handlers::rpc::request;
use axum::{Router, routing::post, serve};
use eyre::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

pub struct Server {
    socket: SocketAddr,
}

impl Server {
    pub async fn new(socket: SocketAddr) -> Result<Self> {
        Ok(Self { socket })
    }

    pub async fn run(self) -> Result<()> {
        let listener = TcpListener::bind(self.socket).await?;

        let app = Router::new().route("/", post(request));

        info!(socket = self.socket.to_string(), "Starting router");

        serve(listener, app).await?;

        Ok(())
    }
}
