mod routes;
mod server;

use eyre::Result;
use server::Server;
use std::{net::SocketAddr, str::FromStr};
use telemetry::{Telemetry, cleanup};

#[tokio::main]
async fn main() -> Result<()> {
    let Telemetry {
        logger_provider,
        meter_provider,
        profiling_agent,
        tracer_provider,
    } = Telemetry::init("salvo")?;

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
