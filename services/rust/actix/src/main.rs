mod routes;
mod server;

use app::config::AppConfig;
use eyre::Result;
use server::Server;
use std::{net::SocketAddr, str::FromStr};
use telemetry::{Telemetry, cleanup};

#[actix_web::main]
async fn main() -> Result<()> {
    let service = std::env::var("SERVICE")?;

    let Telemetry {
        logger_provider,
        meter_provider,
        profiling_agent,
        tracer_provider,
    } = Telemetry::init(&service)?;

    let socket = std::env::var("SOCKET")?;
    let socket = SocketAddr::from_str(&socket)?;
    let app_config = AppConfig::new()?;

    let server = Server::new(socket, app_config);

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
