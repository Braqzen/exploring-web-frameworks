mod api;
mod router;
mod server;
mod state;
mod task;

use eyre::Result;
use rust_telemetry::{Telemetry, cleanup};
use server::Server;
use std::{net::SocketAddr, str::FromStr};

#[rocket::main]
async fn main() -> Result<()> {
    let Telemetry {
        logger_provider,
        meter_provider,
        profiling_agent,
        tracer_provider,
    } = Telemetry::init("rocket")?;

    let socket = std::env::var("SOCKET")?;
    let socket = SocketAddr::from_str(&socket)?;

    let server = Server::new(socket);

    let profiling_agent = profiling_agent.start()?;

    let result = server.run().await;
    cleanup(
        &logger_provider,
        &meter_provider,
        profiling_agent,
        &tracer_provider,
    );

    result
}
