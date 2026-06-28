use crate::{router::router, state::State};
use eyre::Result;
use rocket::Config;
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
        let app = router(self.state);

        info!(socket = self.socket.to_string(), "Starting router");

        app.configure(
            Config::figment()
                .merge((Config::ADDRESS, self.socket.ip()))
                .merge((Config::PORT, self.socket.port())),
        )
        .launch()
        .await?;

        Ok(())
    }
}
